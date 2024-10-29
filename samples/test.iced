// Create a top level container
{<bg:gradient(0.8,[#202030@0.0, #404045@0.3, #323030@1.0]), text-color:#ffffff>

    // Create a column for the top bar, and bottom content
    col<padding:5>[
        // Top bar container with a fixed height, and filling the width
        {<align-x:center, align-y:center, padding:10,10,10,10, height:100, width:fill, border:color(#0090a0),width(1.0),radius(6.0), bg:color(#2020f009)>
            row<align:center, spacing:10>[
                image#ferris(file!("samples/ferris.png")),
                text#title<size:40, text-color:#c0c0c0, wrapping:none, shaping:advanced>("Snowcap Viewer")
            ]
        },

        // Bottom container
        {
            row<height:fill, padding:top(2), spacing:2>[
                // Left column container
                {<width:200, height:fill, align-x:center, padding: 10.0, border:color(#0090a0),width(1),radius(10)>
                    // Left column
                    col[
                        svg(file!("samples/coder.svg")),
                        pick-list#foo<selected:"abc">(["abc", "bar"]),
                        pick-list#bar<selected:"bar">(["baz", "bar"]),
                        text<size:24>("I'm some text"),
                        text<size:10>("More text in a Column"),
                        text(url!("http://icanhazip.com")),
                        image(url!("https://picsum.photos/200/300")),
                        space<size:10>(),
                        {<height:fill, align-y:center>
                            text<size:17>("Edit the test.iced file to see your changes hot reloaded")
                        }
                    ]
                },

                // Middle Column container
                {<width:fill, height:fill, align-x:center, align-y:top, padding:10.0, border:color(#0090a0), width(1), radius(10)>
                    // Middle column (shorthand |)
                    |<align:center>[
                        markdown(file!("README.md")),
                        qr-code<cell-size:10>(qr!("https://iced.rs")),
                        button#my-button(text<size:20>("Button")),
                        toggler#toggle-a<toggled:false, label:"Foo", size:20>(),
                        toggler#toggle-b<toggled:false, label:"Bar", size:30>(),
                        toggler<toggled:false, label:"Baz", size:40>()
                    ]
                },

                // Right Column
                {<width:200, height:fill, align-x:left, padding:10.0, border:color(#0090a0), width(1), radius(10)>
                    |[
                        text<size:30>("Ipsum"),
                        text(url!("http://corporatelorem.kovah.de/api/3?format=text"))
                    ]
                }
            ]
        }
    ]
}