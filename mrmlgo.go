package mrmlgo

/*
#cgo linux CFLAGS: -I./
#cgo linux,amd64 LDFLAGS: -L./libs/linux_x64 -lmrml_capi -lm -ldl -lpthread -lgcc_s -lc -Wl,--allow-multiple-definition
#cgo linux,386 LDFLAGS: -L./libs/linux_x86 -lmrml_capi -lm -ldl -lpthread -lgcc_s -lc -Wl,--allow-multiple-definition
#cgo linux,arm64 LDFLAGS: -L./libs/linux_arm64 -lmrml_capi -lm -ldl -lpthread -lgcc_s -lc -Wl,--allow-multiple-definition
#cgo windows,amd64 LDFLAGS: ./libs/windows_x64/libmrml_capi.a -lkernel32 -ladvapi32 -lntdll -luserenv -lws2_32 -ldbghelp
#cgo windows,386 LDFLAGS: ./libs/windows_x86/libmrml_capi.a -lkernel32 -ladvapi32 -lntdll -luserenv -lws2_32 -ldbghelp
#cgo darwin,arm64 LDFLAGS: -L./libs/darwin_arm64 -lmrml_capi
#cgo darwin,amd64 LDFLAGS: -L./libs/darwin_x64 -lmrml_capi
#include "./mrml-capi/include/mrml-capi.h"
*/
import "C"
import (
	"errors"
	"runtime"
	"unsafe"
)

type ParserOptions struct {
	cParserOptions *C.ParserOptions
	includeLoaders map[string]string
}

type ParserOutput struct {
	cParserOutput *C.ParserOutput
}

func NewParseOptions(includeLoaders map[string]string) (*ParserOptions, error) {
	c := &ParserOptions{
		includeLoaders: includeLoaders,
	}

	C.new_parser_options(&c.cParserOptions)

	// Add Loaders
	for key, value := range includeLoaders {
		cKey := C.CString(key)
		defer C.free(unsafe.Pointer(cKey))
		cValue := C.CString(value)
		defer C.free(unsafe.Pointer(cValue))

		C.add_memory_loader(c.cParserOptions, cKey, cValue)
	}

	runtime.SetFinalizer(c, (*ParserOptions).Destory)
	return c, nil
}

func (c *ParserOptions) ParseJSON(input string) (*ParserOutput, error) {
	cGoInput := C.CString(input)
	defer C.free(unsafe.Pointer(cGoInput))

	runtime.KeepAlive(c)

	ret := &ParserOutput{}

	results := C.parse_json(c.cParserOptions, cGoInput, &ret.cParserOutput)
	if results != C.ParserSuccess {
		return nil, errors.New(C.GoString(C.mrml_last_error()))
	}

	runtime.SetFinalizer(ret, (*ParserOutput).Destory)
	return ret, nil
}

func (c *ParserOptions) ParseMJML(input string) (*ParserOutput, error) {
	cGoInput := C.CString(input)
	defer C.free(unsafe.Pointer(cGoInput))

	runtime.KeepAlive(c)

	ret := &ParserOutput{}

	results := C.parse_mjml(c.cParserOptions, cGoInput, &ret.cParserOutput)
	if results != C.ParserSuccess {
		return nil, errors.New(C.GoString(C.mrml_last_error()))
	}

	runtime.SetFinalizer(ret, (*ParserOutput).Destory)
	return ret, nil
}

func (c *ParserOutput) ToHTML() (string, error) {
	var ret *C.char
	defer C.free(unsafe.Pointer(ret))

	results := C.to_html(c.cParserOutput, &ret)
	if results != C.RenderSuccess {
		return "", errors.New("rendering error")
	}

	return C.GoString(ret), nil
}

func (c *ParserOutput) ToJSON() (string, error) {
	var ret *C.char
	defer C.free(unsafe.Pointer(ret))

	results := C.to_json(c.cParserOutput, &ret)
	if results != C.RenderSuccess {
		return "", errors.New("rendering error")
	}

	return C.GoString(ret), nil
}

func (c *ParserOutput) ToMJML() (string, error) {
	var ret *C.char
	defer C.free(unsafe.Pointer(ret))

	results := C.to_mjml(c.cParserOutput, &ret)
	if results != C.RenderSuccess {
		return "", errors.New("rendering error")
	}

	return C.GoString(ret), nil
}

func (c *ParserOutput) Destory() {
	if c.cParserOutput != nil {
		C.destroy_parser_output(c.cParserOutput)
		c.cParserOutput = nil
	}

	runtime.SetFinalizer(c, nil)
}

func (c *ParserOptions) Destory() {
	if c.cParserOptions != nil {
		C.destroy_parser_options(c.cParserOptions)
		c.cParserOptions = nil
	}
	runtime.SetFinalizer(c, nil)
}
