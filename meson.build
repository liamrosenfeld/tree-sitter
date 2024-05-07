project('tree_sitter', 'c', default_options: ['c_std=gnu99'])

tree_sitter_lib = library(
  'tree_sitter',
  'lib/src/alloc.c',
  'lib/src/get_changed_ranges.c',
  'lib/src/lib.c',
  'lib/src/language.c',
  'lib/src/lexer.c',
  'lib/src/node.c',
  'lib/src/parser.c',
  'lib/src/query.c',
  'lib/src/stack.c',
  'lib/src/subtree.c',
  'lib/src/tree.c',
  'lib/src/tree_cursor.c',
  'lib/src/wasm_store.c',
  include_directories: ['lib/include']
)

tree_sitter_dep = declare_dependency(
  link_with: [tree_sitter_lib],
  include_directories: ['lib/include']
)
