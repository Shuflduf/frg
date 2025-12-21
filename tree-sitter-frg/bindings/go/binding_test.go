package tree_sitter_frg_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_frg "github.com/shuflduf/frg/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_frg.Language())
	if language == nil {
		t.Errorf("Error loading Frg grammar")
	}
}
