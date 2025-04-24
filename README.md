# MRML Go

[![Go Reference](https://pkg.go.dev/badge/github.com/codedninja/mrmlgo.svg)](https://pkg.go.dev/github.com/codedninja/mrmlgo)
[![GitHub License](https://img.shields.io/github/license/codedninja/mrmlgo)](https://opensource.org/licenses/MIT)

A Go wrapper for MRML, the MJML markup language implemented in rust.

## Installation

```bash
go get github.com/codedninja/mrmlgo
```

## Usage

```go
package main

import (
	"fmt"
	"github.com/codedninja/mrmlgo"
)

func main() {
    mjml := `
    <mjml>
        <mj-body>
        <mj-section>
        <mj-column>
            <mj-image width="100px" src="/assets/img/logo-small.png"></mj-image>

            <mj-divider border-color="#F45E43"></mj-divider>

            <mj-text font-size="20px" color="#F45E43" font-family="helvetica">Hello World</mj-text>
        </mj-column>
        </mj-section>
    </mj-body>
    </mjml>`


    options, _ := mrmlgo.NewParseOptions(map[string]string{})
    parsed, _ := mrmlgo.ParseMJML(mjml)

    html, _ := parsed.ToHTML();
    
    fmt.Println(html)
}
```

### Detailed Usage

At the this current moment `mrmlgo` only supports in memory includes. You can pass a `map[string]string` when creating calling `NewParseOptions`. The format is ``map[filename]contents`.

```go
package main

import (
	"fmt"
	"github.com/codedninja/mrmlgo"
)

func main() {
	includes := map[string]string{
		"hello-world.mjml": "<mj-text>Hello World</mj-text>",
	}

    mjml := `
    <mjml>
        <mj-body>
            <mj-include path="hello-world.mjml" />
        </mj-body>
    </mjml>`

	options, _ := mrmlgo.NewParseOptions(includes)
    parsed, _ := options.ParseMJML(mjml); 
    output, _ := parsed.ToHTML();

	fmt.Println("Output:", output)
}
```
## Contributing

Coming soon.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [mrml](https://github.com/jdrouet/mrml) - MJML Crate
- [mjml](http://mjml.io/) - The only framework that makes responsive emails easy.

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/codedninja/mrmlgo/issues).

## TODO

- Github workflow for building rust libraries
- Add tests