use luaparse_rs::{Parser, Luau, ast};
use luaparse_rs::ast::visitor::{Visitor, VisitorMut, walk_stmt, walk_expr};
use luaparse_rs::ast::{
    Stmt, StmtKind, Expr, ExprKind, Identifier, Block,
};

#[cfg(feature = "lua51")]
use luaparse_rs::Lua51;

fn parse_luau(input: &str) -> ast::Ast {
    let parser = Parser::<Luau>::new(input).unwrap();
    parser.parse().unwrap()
}

#[test]
fn test_count_identifiers() {
    let ast = parse_luau("local x = 1\nlocal y = x + 2");
    let mut count = 0;
    ast.for_each_identifier(|_| count += 1);
    // x (decl), y (decl), x (ref)
    assert_eq!(count, 3);
}

#[test]
fn test_count_function_calls() {
    let ast = parse_luau("print(foo(1, 2), bar())");
    let mut call_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::Call(_)) {
            call_count += 1;
        }
    });
    // print(...), foo(1,2), bar()
    assert_eq!(call_count, 3);
}

#[test]
fn test_count_statements() {
    let ast = parse_luau("local a = 1\nlocal b = 2\nreturn a + b");
    let mut stmt_count = 0;
    ast.for_each_stmt(|_| stmt_count += 1);
    assert_eq!(stmt_count, 3);
}

#[test]
fn test_collect_identifier_names() {
    let ast = parse_luau("local foo = bar + baz");
    let mut names = vec![];
    ast.for_each_identifier(|ident| {
        names.push(ident.name.clone());
    });
    assert!(names.contains(&"foo".into()));
    assert!(names.contains(&"bar".into()));
    assert!(names.contains(&"baz".into()));
}

#[test]
fn test_visitor_trait_custom_impl() {
    struct NumberCounter {
        count: usize,
    }

    impl Visitor for NumberCounter {
        fn visit_expr(&mut self, expr: &Expr) {
            if matches!(expr.kind, ExprKind::Number(_)) {
                self.count += 1;
            }
            walk_expr(self, expr);
        }
    }

    let ast = parse_luau("local x = 1 + 2 + 3");
    let mut counter = NumberCounter { count: 0 };
    counter.visit_ast(&ast);
    assert_eq!(counter.count, 3);
}

#[test]
fn test_visitor_nested_blocks() {
    struct BlockCounter {
        count: usize,
    }

    impl Visitor for BlockCounter {
        fn visit_block(&mut self, block: &Block) {
            self.count += 1;
            ast::visitor::walk_block(self, block);
        }
    }

    let ast = parse_luau(
        "if true then\n  while false do\n    local x = 1\n  end\nend",
    );
    let mut counter = BlockCounter { count: 0 };
    counter.visit_ast(&ast);
    // top-level block, if-then block, while body block
    assert_eq!(counter.count, 3);
}

#[test]
fn test_visitor_function_params() {
    let ast = parse_luau("local function foo(a, b, c) end");
    let mut param_count = 0;
    struct ParamCounter<'a>(&'a mut usize);
    impl Visitor for ParamCounter<'_> {
        fn visit_parameter(&mut self, param: &luaparse_rs::ast::Parameter) {
            *self.0 += 1;
            ast::visitor::walk_parameter(self, param);
        }
    }
    let mut counter = ParamCounter(&mut param_count);
    counter.visit_ast(&ast);
    assert_eq!(param_count, 3);
}

#[test]
fn test_visitor_table_fields() {
    let ast = parse_luau("local t = {a = 1, b = 2, 3}");
    let mut field_count = 0;
    struct FieldCounter<'a>(&'a mut usize);
    impl Visitor for FieldCounter<'_> {
        fn visit_table_field(&mut self, field: &ast::TableField) {
            *self.0 += 1;
            ast::visitor::walk_table_field(self, field);
        }
    }
    let mut counter = FieldCounter(&mut field_count);
    counter.visit_ast(&ast);
    assert_eq!(field_count, 3);
}

#[test]
fn test_visitor_method_calls() {
    let ast = parse_luau("obj:method1():method2()");
    let mut method_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::MethodCall(_)) {
            method_count += 1;
        }
    });
    assert_eq!(method_count, 2);
}

#[test]
fn test_visitor_binary_exprs() {
    let ast = parse_luau("local x = 1 + 2 * 3 - 4");
    let mut binop_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::Binary(_)) {
            binop_count += 1;
        }
    });
    // (1 + (2 * 3)) - 4 = 3 binary ops
    assert_eq!(binop_count, 3);
}

#[test]
fn test_visitor_for_loops() {
    let ast = parse_luau(
        "for i = 1, 10 do end\nfor k, v in pairs(t) do end",
    );
    let mut numeric_for = 0;
    let mut generic_for = 0;
    ast.for_each_stmt(|stmt| {
        match &stmt.kind {
            StmtKind::NumericForLoop(_) => numeric_for += 1,
            StmtKind::GenericForLoop(_) => generic_for += 1,
            _ => {}
        }
    });
    assert_eq!(numeric_for, 1);
    assert_eq!(generic_for, 1);
}

#[test]
fn test_visitor_if_expression_luau() {
    let ast = parse_luau("local x = if true then 1 else 2");
    let mut if_expr_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::IfExpression(_)) {
            if_expr_count += 1;
        }
    });
    assert_eq!(if_expr_count, 1);
}

#[test]
fn test_visitor_interpolated_string() {
    let ast = parse_luau("local x = `hello {name} world {age}`");
    let mut interp_count = 0;
    let mut inner_expr_count = 0;
    ast.for_each_expr(|expr| {
        match &expr.kind {
            ExprKind::InterpolatedString(interp) => {
                interp_count += 1;
                for seg in &interp.segments {
                    if matches!(seg, ast::InterpolationSegment::Expression(_)) {
                        inner_expr_count += 1;
                    }
                }
            }
            _ => {}
        }
    });
    assert_eq!(interp_count, 1);
    assert_eq!(inner_expr_count, 2);
}

#[test]
fn test_visitor_assignment_targets() {
    let ast = parse_luau("a, b.c, d[1] = 1, 2, 3");
    let mut target_count = 0;
    struct TargetCounter<'a>(&'a mut usize);
    impl Visitor for TargetCounter<'_> {
        fn visit_assignment_target(&mut self, target: &ast::AssignmentTarget) {
            *self.0 += 1;
            ast::visitor::walk_assignment_target(self, target);
        }
    }
    let mut counter = TargetCounter(&mut target_count);
    counter.visit_ast(&ast);
    assert_eq!(target_count, 3);
}

#[test]
fn test_visitor_return_values() {
    let ast = parse_luau("return 1, 2, 3");
    let mut return_val_count = 0;
    ast.for_each_stmt(|stmt| {
        if let StmtKind::ReturnStatement(ret) = &stmt.kind {
            return_val_count = ret.values.len();
        }
    });
    assert_eq!(return_val_count, 3);
}

#[test]
fn test_visitor_deeply_nested() {
    let ast = parse_luau(
        "if true then\n  if false then\n    if true then\n      local x = 1\n    end\n  end\nend",
    );
    let mut deepest_local = false;
    ast.for_each_stmt(|stmt| {
        if matches!(stmt.kind, StmtKind::LocalDeclaration(_)) {
            deepest_local = true;
        }
    });
    assert!(deepest_local);
}

#[test]
fn test_visitor_empty_program() {
    let ast = parse_luau("");
    let mut count = 0;
    ast.for_each_stmt(|_| count += 1);
    assert_eq!(count, 0);
}

#[test]
fn test_visitor_comments_visited() {
    struct CommentCounter {
        count: usize,
    }
    impl Visitor for CommentCounter {
        fn visit_comment(&mut self, _comment: &ast::Comment) {
            self.count += 1;
        }
    }
    let ast = parse_luau("-- a comment\nlocal x = 1 -- inline");
    let mut counter = CommentCounter { count: 0 };
    counter.visit_ast(&ast);
    assert_eq!(counter.count, ast.comments.len());
}

#[test]
fn test_visitor_mut_rename_identifiers() {
    struct Renamer;
    impl VisitorMut for Renamer {
        fn visit_identifier(&mut self, ident: &mut Identifier) {
            if ident.name == "foo" {
                ident.name = "bar".into();
            }
        }
    }

    let mut ast = parse_luau("local foo = foo + 1");
    let mut renamer = Renamer;
    renamer.visit_ast(&mut ast);

    let mut found_foo = false;
    let mut found_bar = false;
    ast.for_each_identifier(|ident| {
        if ident.name == "foo" {
            found_foo = true;
        }
        if ident.name == "bar" {
            found_bar = true;
        }
    });
    assert!(!found_foo, "should have renamed all 'foo'");
    assert!(found_bar, "should have 'bar' now");
}

#[test]
fn test_visitor_mut_replace_numbers() {
    struct NumberReplacer;
    impl VisitorMut for NumberReplacer {
        fn visit_expr(&mut self, expr: &mut Expr) {
            if let ExprKind::Number(ref mut num) = expr.kind {
                num.raw = "42".into();
            }
            ast::visitor::walk_expr_mut(self, expr);
        }
    }

    let mut ast = parse_luau("local x = 1 + 2 + 3");
    let mut replacer = NumberReplacer;
    replacer.visit_ast(&mut ast);

    let mut all_42 = true;
    ast.for_each_expr(|expr| {
        if let ExprKind::Number(num) = &expr.kind {
            if num.raw != "42" {
                all_42 = false;
            }
        }
    });
    assert!(all_42, "all numbers should be 42 now");
}

#[test]
fn test_visitor_mut_inject_statements() {
    struct BlockInjector;
    impl VisitorMut for BlockInjector {
        fn visit_block(&mut self, block: &mut Block) {
            let break_stmt = Stmt::synthetic(StmtKind::BreakStatement);
            block.statements.push(break_stmt);
        }
    }

    let mut ast = parse_luau("local x = 1");
    let original_count = ast.block.statements.len();
    let mut injector = BlockInjector;
    injector.visit_ast(&mut ast);
    assert_eq!(ast.block.statements.len(), original_count + 1);
}

#[test]
fn test_visitor_mut_function_body() {
    struct BodyClearer;
    impl VisitorMut for BodyClearer {
        fn visit_stmt(&mut self, stmt: &mut Stmt) {
            if let StmtKind::LocalFunctionDeclaration(ref mut decl) = stmt.kind {
                decl.body.statements.clear();
            }
            ast::visitor::walk_stmt_mut(self, stmt);
        }
    }

    let mut ast = parse_luau("local function foo()\n  local x = 1\n  return x\nend");
    let mut clearer = BodyClearer;
    clearer.visit_ast(&mut ast);

    if let StmtKind::LocalFunctionDeclaration(ref decl) = ast.block.statements[0].kind {
        assert_eq!(decl.body.statements.len(), 0);
    } else {
        panic!("expected local function declaration");
    }
}

#[test]
fn test_visitor_scope_analysis_pattern() {
    struct ScopeAnalyzer {
        declarations: Vec<String>,
        references: Vec<String>,
    }

    impl Visitor for ScopeAnalyzer {
        fn visit_stmt(&mut self, stmt: &Stmt) {
            if let StmtKind::LocalDeclaration(ref decl) = stmt.kind {
                for var in &decl.names {
                    self.declarations.push(var.name.name.clone());
                }
            }
            walk_stmt(self, stmt);
        }

        fn visit_expr(&mut self, expr: &Expr) {
            if let ExprKind::Identifier(ref ident) = expr.kind {
                self.references.push(ident.name.clone());
            }
            walk_expr(self, expr);
        }
    }

    let ast = parse_luau("local x = 1\nlocal y = x + 2\nreturn y");
    let mut analyzer = ScopeAnalyzer {
        declarations: vec![],
        references: vec![],
    };
    analyzer.visit_ast(&ast);

    assert_eq!(analyzer.declarations, vec!["x", "y"]);
    assert!(analyzer.references.contains(&"x".to_string()));
    assert!(analyzer.references.contains(&"y".to_string()));
}

#[test]
fn test_visitor_complexity_counter() {
    struct ComplexityCounter {
        complexity: usize,
    }

    impl Visitor for ComplexityCounter {
        fn visit_stmt(&mut self, stmt: &Stmt) {
            match &stmt.kind {
                StmtKind::IfStatement(if_stmt) => {
                    self.complexity += 1; // if
                    self.complexity += if_stmt.elseif_branches.len(); // elseifs
                }
                StmtKind::WhileLoop(_) | StmtKind::RepeatLoop(_)
                | StmtKind::NumericForLoop(_) | StmtKind::GenericForLoop(_) => {
                    self.complexity += 1;
                }
                _ => {}
            }
            walk_stmt(self, stmt);
        }

        fn visit_expr(&mut self, expr: &Expr) {
            if let ExprKind::Binary(ref bin) = expr.kind {
                if matches!(
                    bin.operator,
                    ast::BinaryOperator::And | ast::BinaryOperator::Or
                ) {
                    self.complexity += 1;
                }
            }
            walk_expr(self, expr);
        }
    }

    let ast = parse_luau(
        "if a then\n  while b do end\nend\nfor i = 1, 10 do end",
    );
    let mut counter = ComplexityCounter { complexity: 0 };
    counter.visit_ast(&ast);
    // if + while + for = 3
    assert_eq!(counter.complexity, 3);
}

#[test]
fn test_visitor_mut_obfuscate_identifiers() {
    struct Obfuscator {
        counter: usize,
    }

    impl VisitorMut for Obfuscator {
        fn visit_identifier(&mut self, ident: &mut Identifier) {
            ident.name = format!("_v{}", self.counter);
            self.counter += 1;
        }
    }

    let mut ast = parse_luau("local hello = world");
    let mut obf = Obfuscator { counter: 0 };
    obf.visit_ast(&mut ast);

    // All identifiers should now be _v0, _v1, etc
    let mut names = vec![];
    ast.for_each_identifier(|ident| names.push(ident.name.clone()));
    assert!(names.iter().all(|n| n.starts_with("_v")));
}

#[test]
fn test_visitor_export_statement() {
    let ast = parse_luau("export type Foo = number");
    let mut found_export = false;
    ast.for_each_stmt(|stmt| {
        if matches!(stmt.kind, StmtKind::ExportStatement(_)) {
            found_export = true;
        }
    });
    assert!(found_export);
}

#[test]
fn test_visitor_compound_assignment() {
    let ast = parse_luau("x += 1");
    let mut found_compound = false;
    ast.for_each_stmt(|stmt| {
        if matches!(stmt.kind, StmtKind::CompoundAssignment(_)) {
            found_compound = true;
        }
    });
    assert!(found_compound);
}

#[test]
fn test_visitor_do_block() {
    let ast = parse_luau("do\n  local x = 1\nend");
    let mut found_do = false;
    let mut found_local_inside = false;
    ast.for_each_stmt(|stmt| {
        match &stmt.kind {
            StmtKind::DoBlock(_) => found_do = true,
            StmtKind::LocalDeclaration(_) => found_local_inside = true,
            _ => {}
        }
    });
    assert!(found_do);
    assert!(found_local_inside);
}

#[test]
fn test_visitor_unary_expressions() {
    let ast = parse_luau("local x = -1\nlocal y = not true\nlocal z = #t");
    let mut unary_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::Unary(_)) {
            unary_count += 1;
        }
    });
    assert_eq!(unary_count, 3);
}

#[test]
fn test_visitor_string_literals() {
    let ast = parse_luau(r#"local a = "hello" local b = 'world'"#);
    let mut string_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::String(_)) {
            string_count += 1;
        }
    });
    assert_eq!(string_count, 2);
}

#[test]
fn test_visitor_field_access_chain() {
    let ast = parse_luau("local x = a.b.c.d");
    let mut field_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::FieldAccess(_)) {
            field_count += 1;
        }
    });
    // a.b, (a.b).c, ((a.b).c).d = 3 field accesses
    assert_eq!(field_count, 3);
}

#[test]
fn test_visitor_parenthesized() {
    let ast = parse_luau("local x = (1 + 2)");
    let mut paren_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::Parenthesized(_)) {
            paren_count += 1;
        }
    });
    assert_eq!(paren_count, 1);
}

#[test]
fn test_visitor_continue_statement() {
    let ast = parse_luau("for i = 1, 10 do continue end");
    let mut found_continue = false;
    ast.for_each_stmt(|stmt| {
        if matches!(stmt.kind, StmtKind::ContinueStatement) {
            found_continue = true;
        }
    });
    assert!(found_continue);
}

#[test]
fn test_visitor_repeat_loop() {
    let ast = parse_luau("repeat local x = 1 until x > 10");
    let mut found_repeat = false;
    ast.for_each_stmt(|stmt| {
        if matches!(stmt.kind, StmtKind::RepeatLoop(_)) {
            found_repeat = true;
        }
    });
    assert!(found_repeat);
}

#[test]
fn test_visitor_function_expression() {
    let ast = parse_luau("local f = function(a, b) return a + b end");
    let mut func_expr_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::Function(_)) {
            func_expr_count += 1;
        }
    });
    assert_eq!(func_expr_count, 1);
}

#[test]
fn test_visitor_vararg() {
    let ast = parse_luau("local function f(...) return ... end");
    let mut vararg_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::Vararg) {
            vararg_count += 1;
        }
    });
    assert_eq!(vararg_count, 1);
}

#[test]
fn test_visitor_index_access() {
    let ast = parse_luau("local x = t[1][2]");
    let mut index_count = 0;
    ast.for_each_expr(|expr| {
        if matches!(expr.kind, ExprKind::IndexAccess(_)) {
            index_count += 1;
        }
    });
    assert_eq!(index_count, 2);
}

#[cfg(feature = "lua51")]
#[test]
fn test_visitor_goto_and_label() {
    // goto/labels are Lua 5.2+, test with lua51 feature that they're rejected
    let input = "::label::\ngoto label";
    let parser = Parser::<luaparse_rs::Lua52>::new(input).unwrap();
    let ast = parser.parse().unwrap();
    let mut goto_count = 0;
    let mut label_count = 0;
    ast.for_each_stmt(|stmt| {
        match &stmt.kind {
            StmtKind::GotoStatement(_) => goto_count += 1,
            StmtKind::LabelStatement(_) => label_count += 1,
            _ => {}
        }
    });
    assert_eq!(goto_count, 1);
    assert_eq!(label_count, 1);
}

#[test]
fn test_visitor_while_loop() {
    let ast = parse_luau("while true do break end");
    let mut while_count = 0;
    let mut break_count = 0;
    ast.for_each_stmt(|stmt| {
        match &stmt.kind {
            StmtKind::WhileLoop(_) => while_count += 1,
            StmtKind::BreakStatement => break_count += 1,
            _ => {}
        }
    });
    assert_eq!(while_count, 1);
    assert_eq!(break_count, 1);
}

#[test]
fn test_visitor_elseif_branches() {
    let ast = parse_luau(
        "if a then\nelseif b then\nelseif c then\nelse\nend",
    );
    let mut elseif_count = 0;
    ast.for_each_stmt(|stmt| {
        if let StmtKind::IfStatement(ref if_stmt) = stmt.kind {
            elseif_count = if_stmt.elseif_branches.len();
        }
    });
    assert_eq!(elseif_count, 2);
}

#[test]
fn test_visitor_mut_walk_all_nodes() {
    struct NoopMut;
    impl VisitorMut for NoopMut {}

    let code = r#"
        local function greet(name: string): string
            return `hello {name}`
        end
        local x = if true then 1 else 2
        export type Foo = { bar: number }
        for i = 1, 10 do
            x += i
        end
        while true do break end
        repeat until false
        do end
    "#;
    let mut ast = parse_luau(code);
    let mut noop = NoopMut;
    noop.visit_ast(&mut ast);
}

#[test]
fn test_visitor_walk_all_nodes_immutable() {
    struct Noop;
    impl Visitor for Noop {}

    let code = r#"
        local function greet(name: string): string
            return `hello {name}`
        end
        local x = if true then 1 else 2
        export type Foo = { bar: number }
        for i = 1, 10 do
            x += i
        end
        for k, v in pairs(t) do end
        while true do break end
        repeat until false
        do end
        a.b.c = 1
        obj:method()
        local t = {a = 1, [2] = 3, 4}
        local y = -x
        local z = not true
        local w = #t
        local p = (1 + 2)
        local q: number = 1 :: number
    "#;
    let ast = parse_luau(code);
    let mut noop = Noop;
    noop.visit_ast(&ast);
}