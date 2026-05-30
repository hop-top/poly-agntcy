package main

import (
	"bytes"
	"testing"
)

func TestRootHelp(t *testing.T) {
	cmd := newRootCmd()
	cmd.SetArgs([]string{"--help"})
	var buf bytes.Buffer
	cmd.SetOut(&buf)
	if err := cmd.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	out := buf.String()
	for _, want := range []string{"register", "discover", "describe", "publish", "verify"} {
		if !bytes.Contains(buf.Bytes(), []byte(want)) {
			t.Fatalf("help must mention %s: %s", want, out)
		}
	}
}
