//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod passing {
    use crate::css;
    use reqwest::blocking::Client;
    use std::collections::HashMap;

    #[test]
    fn empty_input() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        assert_eq!(
            css::embed_css(cache, &client, "", "", false, false, false,),
            ""
        );
    }

    #[test]
    fn style_exclude_unquoted_images() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const STYLE: &str = "/* border: none;*/\
            background-image: url(https://somewhere.com/bg.png); \
            list-style: url(/assets/images/bullet.svg);\
            width:99.998%; \
            margin-top: -20px; \
            line-height: -1; \
            height: calc(100vh - 10pt)";

        assert_eq!(
            css::embed_css(
                cache,
                &client,
                "https://doesntmatter.local/",
                &STYLE,
                false,
                true,
                true,
            ),
            format!(
                "/* border: none;*/\
                background-image: url('{empty_image}'); \
                list-style: url('{empty_image}');\
                width:99.998%; \
                margin-top: -20px; \
                line-height: -1; \
                height: calc(100vh - 10pt)",
                empty_image = empty_image!()
            )
        );
    }

    #[test]
    fn style_exclude_single_quoted_images() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const STYLE: &str = "/* border: none;*/\
            background-image: url('https://somewhere.com/bg.png'); \
            list-style: url('/assets/images/bullet.svg');\
            width:99.998%; \
            margin-top: -20px; \
            line-height: -1; \
            height: calc(100vh - 10pt)";

        assert_eq!(
            css::embed_css(cache, &client, "", &STYLE, false, true, true,),
            format!(
                "/* border: none;*/\
                background-image: url('{empty_image}'); \
                list-style: url('{empty_image}');\
                width:99.998%; \
                margin-top: -20px; \
                line-height: -1; \
                height: calc(100vh - 10pt)",
                empty_image = empty_image!()
            )
        );
    }

    #[test]
    fn style_block() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const CSS: &str = "\
            #id.class-name:not(:nth-child(3n+0)) {\n  \
            // border: none;\n  \
            background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=');\n\
            }\n\
            \n\
            html > body {}";

        assert_eq!(
            css::embed_css(cache, &client, "file:///", &CSS, false, false, true,),
            CSS
        );
    }

    #[test]
    fn attribute_selectors() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const CSS: &str = "\
            [data-value] {
                /* Attribute exists */
            }

            [data-value='foo'] {
                /* Attribute has this exact value */
            }

            [data-value*='foo'] {
                /* Attribute value contains this value somewhere in it */
            }

            [data-value~='foo'] {
                /* Attribute has this value in a space-separated list somewhere */
            }

            [data-value^='foo'] {
                /* Attribute value starts with this */
            }

            [data-value|='foo'] {
                /* Attribute value starts with this in a dash-separated list */
            }

            [data-value$='foo'] {
                /* Attribute value ends with this */
            }
            ";

        assert_eq!(
            css::embed_css(cache, &client, "", &CSS, false, false, false,),
            CSS
        );
    }

    #[test]
    fn import_string() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const CSS: &str = "\
            @charset 'UTF-8';\n\
            \n\
            @import 'data:text/css,html{background-color:%23000}';\n\
            \n\
            @import url('data:text/css,html{color:%23fff}')\n\
            ";

        assert_eq!(
            css::embed_css(
                cache,
                &client,
                "https://doesntmatter.local/",
                &CSS,
                false,
                false,
                true,
            ),
            "\
            @charset 'UTF-8';\n\
            \n\
            @import 'data:text/css;base64,aHRtbHtiYWNrZ3JvdW5kLWNvbG9yOiMwMDB9';\n\
            \n\
            @import url('data:text/css;base64,aHRtbHtjb2xvcjojZmZmfQ==')\n\
            "
        );
    }

    #[test]
    fn hash_urls() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const CSS: &str = "\
            body {\n    \
                behavior: url(#default#something);\n\
            }\n\
            \n\
            .scissorHalf {\n    \
                offset-path: url(#somePath);\n\
            }\n\
            ";

        assert_eq!(
            css::embed_css(
                cache,
                &client,
                "https://doesntmatter.local/",
                &CSS,
                false,
                false,
                true,
            ),
            CSS
        );
    }

    #[test]
    fn transform_percentages_and_degrees() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const CSS: &str = "\
            div {\n    \
                transform: translate(-50%, -50%) rotate(-45deg);\n\
                transform: translate(50%, 50%) rotate(45deg);\n\
                transform: translate(+50%, +50%) rotate(+45deg);\n\
            }\n\
            ";

        assert_eq!(
            css::embed_css(
                cache,
                &client,
                "https://doesntmatter.local/",
                &CSS,
                false,
                false,
                true,
            ),
            CSS
        );
    }

    #[test]
    fn unusual_indents() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const CSS: &str = "\
            .is\\:good:hover {\n    \
                color: green\n\
            }\n\
            \n\
            #\\~\\!\\@\\$\\%\\^\\&\\*\\(\\)\\+\\=\\,\\.\\/\\\\\\'\\\"\\;\\:\\?\\>\\<\\[\\]\\{\\}\\|\\`\\# {\n    \
                color: black\n\
            }\n\
            ";

        assert_eq!(
            css::embed_css(
                cache,
                &client,
                "https://doesntmatter.local/",
                &CSS,
                false,
                false,
                true,
            ),
            CSS
        );
    }

    #[test]
    fn exclude_fonts() {
        let cache = &mut HashMap::new();
        let client = Client::new();

        const CSS: &str = "\
            @font-face {\n    \
                font-family: 'My Font';\n    \
                src: url(my_font.woff);\n\
            }\n\
            \n\
            #identifier {\n    \
                font-family: 'My Font' Arial\n\
            }\n\
            \n\
            @font-face {\n    \
                font-family: 'My Font';\n    \
                src: url(my_font.woff);\n\
            }\n\
            \n\
            div {\n    \
                font-family: 'My Font' Verdana\n\
            }\n\
            ";

        const CSS_OUT: &str = " \
            \n\
            \n\
            #identifier {\n    \
                font-family: 'My Font' Arial\n\
            }\n\
            \n \
            \n\
            \n\
            div {\n    \
                font-family: 'My Font' Verdana\n\
            }\n\
            ";

        assert_eq!(
            css::embed_css(
                cache,
                &client,
                "https://doesntmatter.local/",
                &CSS,
                true,
                false,
                true,
            ),
            CSS_OUT
        );
    }
}
