import XCTest
import SwiftTreeSitter
import TreeSitterFrg

final class TreeSitterFrgTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_frg())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Frg grammar")
    }
}
