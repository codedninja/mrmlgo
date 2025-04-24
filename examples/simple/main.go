package main

import (
	"fmt"
	"log"

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

	options, err := mrmlgo.NewParseOptions(includes)
	if err != nil {
		log.Fatal(err)
	}

	output, err := options.ParseMJML(mjml)
	if err != nil {
		log.Fatal(err)
	}

	out, err := output.ToHTML()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(out)
}
