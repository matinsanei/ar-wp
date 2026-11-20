use crate::ast::*;
use crate::val::{ArrayKey, PhpArray, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct StackFrame {
    pub locals: HashMap<String, Value>,
    pub global_bindings: HashSet<String>,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            global_bindings: HashSet::new(),
        }
    }
}

pub type BuiltinFn = Box<dyn Fn(&mut Engine, Vec<Value>) -> Result<Value, String>>;

pub struct Engine {
    pub globals: HashMap<String, Value>,
    pub functions: HashMap<String, (Vec<(String, Option<Expr>)>, Vec<Stmt>)>,
    pub builtins: HashMap<String, BuiltinFn>,
    pub call_stack: Vec<StackFrame>,
    pub output_buffer: String,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine = Self {
            globals: HashMap::new(),
            functions: HashMap::new(),
            builtins: HashMap::new(),
            call_stack: vec![StackFrame::new()],
            output_buffer: String::new(),
        };
        engine.register_builtins();
        engine
    }

    fn register_builtins(&mut self) {
        // count($arr)
        self.builtins.insert(
            "count".to_string(),
            Box::new(|_engine, args| {
                if let Some(first) = args.first() {
                    match first {
                        Value::Array(arr) => Ok(Value::Int(arr.borrow().len() as i64)),
                        Value::Null => Ok(Value::Int(0)),
                        _ => Ok(Value::Int(1)),
                    }
                } else {
                    Ok(Value::Int(0))
                }
            }),
        );

        // is_array($val)
        self.builtins.insert(
            "is_array".to_string(),
            Box::new(|_engine, args| {
                if let Some(first) = args.first() {
                    Ok(Value::Bool(matches!(first, Value::Array(_))))
                } else {
                    Ok(Value::Bool(false))
                }
            }),
        );

        // is_null($val)
        self.builtins.insert(
            "is_null".to_string(),
            Box::new(|_engine, args| {
                if let Some(first) = args.first() {
                    Ok(Value::Bool(matches!(first, Value::Null)))
                } else {
                    Ok(Value::Bool(true))
                }
            }),
        );

        // isset($val)
        self.builtins.insert(
            "isset".to_string(),
            Box::new(|_engine, args| {
                if let Some(first) = args.first() {
                    Ok(Value::Bool(!matches!(first, Value::Null)))
                } else {
                    Ok(Value::Bool(false))
                }
            }),
        );

        // ksort(&$arr) -> sorts array in place by keys
        self.builtins.insert(
            "ksort".to_string(),
            Box::new(|_engine, args| {
                if let Some(Value::Array(arr)) = args.first() {
                    arr.borrow_mut().sort_keys();
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }),
        );

        // var_dump($val)
        self.builtins.insert(
            "var_dump".to_string(),
            Box::new(|engine, args| {
                for arg in args {
                    let dump = format!("{:#?}\n", arg);
                    engine.output_buffer.push_str(&dump);
                    print!("{}", dump);
                }
                Ok(Value::Null)
            }),
        );
    }

    pub fn get_var(&self, name: &str) -> Value {
        let frame = self.call_stack.last().unwrap();
        if frame.global_bindings.contains(name) || self.call_stack.len() == 1 {
            self.globals.get(name).cloned().unwrap_or(Value::Null)
        } else {
            frame.locals.get(name).cloned().unwrap_or(Value::Null)
        }
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        let frame_idx = self.call_stack.len() - 1;
        let is_global = self.call_stack[frame_idx].global_bindings.contains(name) || self.call_stack.len() == 1;

        if is_global {
            self.globals.insert(name.to_string(), val);
        } else {
            self.call_stack[frame_idx].locals.insert(name.to_string(), val);
        }
    }

    pub fn execute(&mut self, stmts: &[Stmt]) -> Result<Option<Value>, String> {
        for stmt in stmts {
            if let Some(ret) = self.execute_stmt(stmt)? {
                return Ok(Some(ret));
            }
        }
        Ok(None)
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Assign { var, expr } => {
                let val = self.eval_expr(expr)?;
                self.set_var(var, val);
                Ok(None)
            }
            Stmt::ArrayDimAssign { base, dim, expr } => {
                let val = self.eval_expr(expr)?;
                self.assign_array_dim(base, dim.as_ref().map(|d| d.as_ref()), val)?;
                Ok(None)
            }
            Stmt::Global(vars) => {
                let frame = self.call_stack.last_mut().unwrap();
                for var in vars {
                    frame.global_bindings.insert(var.clone());
                    if !self.globals.contains_key(var) {
                        self.globals.insert(var.clone(), Value::Null);
                    }
                }
                Ok(None)
            }
            Stmt::FunctionDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                Ok(None)
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval_expr(cond)?;
                if cond_val.is_truthy() {
                    self.execute(then_branch)
                } else if let Some(else_stmts) = else_branch {
                    self.execute(else_stmts)
                } else {
                    Ok(None)
                }
            }
            Stmt::Foreach {
                expr,
                key_var,
                val_var,
                body,
            } => {
                let val = self.eval_expr(expr)?;
                if let Value::Array(arr) = val {
                    // Clone items to avoid borrow issues during iteration
                    let items: Vec<(ArrayKey, Value)> = {
                        let borrowed = arr.borrow();
                        borrowed.map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
                    };

                    for (k, v) in items {
                        if let Some(kvar) = key_var {
                            let key_value = match k {
                                ArrayKey::Int(i) => Value::Int(i),
                                ArrayKey::Str(s) => Value::String(s),
                            };
                            self.set_var(kvar, key_value);
                        }
                        self.set_var(val_var, v);

                        if let Some(ret) = self.execute(body)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Return(expr_opt) => {
                let val = match expr_opt {
                    Some(e) => self.eval_expr(e)?,
                    None => Value::Null,
                };
                Ok(Some(val))
            }
            Stmt::Echo(exprs) => {
                for expr in exprs {
                    let val = self.eval_expr(expr)?;
                    let text = val.to_string_lossy();
                    self.output_buffer.push_str(&text);
                    print!("{}", text);
                }
                Ok(None)
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }
        }
    }

    /// Handles nested array dimension assignment with auto-vivification:
    /// e.g. `$wp_filter[$tag][$priority][] = $cb;`
    fn assign_array_dim(&mut self, base: &Expr, dim: Option<&Expr>, value: Value) -> Result<(), String> {
        let (root_var, path) = self.extract_array_path(base, dim)?;

        let mut root_val = self.get_var(&root_var);
        if !matches!(root_val, Value::Array(_)) {
            root_val = Value::new_array();
            self.set_var(&root_var, root_val.clone());
        }

        let root_arr = match &root_val {
            Value::Array(a) => a.clone(),
            _ => unreachable!(),
        };

        let mut current_arr = root_arr;

        for (i, segment) in path.iter().enumerate() {
            let is_last = i == path.len() - 1;

            if is_last {
                match segment {
                    Some(key) => {
                        current_arr.borrow_mut().insert(key.clone(), value);
                    }
                    None => {
                        // Empty bracket []
                        current_arr.borrow_mut().push(value);
                    }
                }
                return Ok(());
            }

            // Intermediate segment: auto-vivify if needed
            let key = segment.as_ref().ok_or("Cannot use [] on intermediate array path")?;
            let next_arr = {
                let mut borrowed = current_arr.borrow_mut();
                if let Some(existing) = borrowed.get(key) {
                    if let Value::Array(a) = existing {
                        a.clone()
                    } else {
                        let new_a = Rc::new(RefCell::new(PhpArray::new()));
                        borrowed.insert(key.clone(), Value::Array(new_a.clone()));
                        new_a
                    }
                } else {
                    let new_a = Rc::new(RefCell::new(PhpArray::new()));
                    borrowed.insert(key.clone(), Value::Array(new_a.clone()));
                    new_a
                }
            };
            current_arr = next_arr;
        }

        Ok(())
    }

    fn extract_array_path(
        &mut self,
        base: &Expr,
        last_dim: Option<&Expr>,
    ) -> Result<(String, Vec<Option<ArrayKey>>), String> {
        let mut segments = Vec::new();

        let last_key = match last_dim {
            Some(d) => {
                let val = self.eval_expr(d)?;
                match val {
                    Value::Null => None, // represent empty bracket []
                    Value::Int(i) => Some(ArrayKey::Int(i)),
                    v => Some(ArrayKey::from_str(&v.to_string_lossy())),
                }
            }
            None => None,
        };
        segments.push(last_key);

        let mut curr = base;
        loop {
            match curr {
                Expr::Var(v) => {
                    segments.reverse();
                    return Ok((v.clone(), segments));
                }
                Expr::ArrayAccess { base: inner_base, dim: inner_dim } => {
                    let val = self.eval_expr(inner_dim)?;
                    let key = match val {
                        Value::Null => None,
                        Value::Int(i) => Some(ArrayKey::Int(i)),
                        v => Some(ArrayKey::from_str(&v.to_string_lossy())),
                    };
                    segments.push(key);
                    curr = inner_base;
                }
                _ => return Err("Invalid array access base".to_string()),
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(val) => Ok(val.clone()),
            Expr::Var(name) => Ok(self.get_var(name)),
            Expr::ArrayLiteral(items) => {
                let mut php_arr = PhpArray::new();
                for (key_expr_opt, val_expr) in items {
                    let val = self.eval_expr(val_expr)?;
                    if let Some(key_expr) = key_expr_opt {
                        let key_val = self.eval_expr(key_expr)?;
                        let key = match key_val {
                            Value::Int(i) => ArrayKey::Int(i),
                            v => ArrayKey::from_str(&v.to_string_lossy()),
                        };
                        php_arr.insert(key, val);
                    } else {
                        php_arr.push(val);
                    }
                }
                Ok(Value::Array(Rc::new(RefCell::new(php_arr))))
            }
            Expr::ArrayAccess { base, dim } => {
                let base_val = self.eval_expr(base)?;
                let dim_val = self.eval_expr(dim)?;
                match base_val {
                    Value::Array(arr) => {
                        let key = match dim_val {
                            Value::Int(i) => ArrayKey::Int(i),
                            v => ArrayKey::from_str(&v.to_string_lossy()),
                        };
                        Ok(arr.borrow().get(&key).cloned().unwrap_or(Value::Null))
                    }
                    _ => Ok(Value::Null),
                }
            }
            Expr::BinOp { op, left, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binop(op, l, r)
            }
            Expr::UnaryOp { op, expr } => {
                let val = self.eval_expr(expr)?;
                match op {
                    UnaryOpKind::Not => Ok(Value::Bool(!val.is_truthy())),
                    UnaryOpKind::Neg => match val {
                        Value::Int(i) => Ok(Value::Int(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Ok(Value::Int(-val.to_int_lossy())),
                    },
                }
            }
            Expr::Call { name, args } => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expr(arg)?);
                }
                self.call_function(name, evaluated_args)
            }
            Expr::DynamicCall { callee, args } => {
                let callee_val = self.eval_expr(callee)?;
                let fn_name = callee_val.to_string_lossy();
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expr(arg)?);
                }
                self.call_function(&fn_name, evaluated_args)
            }
        }
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        // 1. Check built-in functions
        if let Some(builtin) = self.builtins.get(name) {
            let builtin_clone = unsafe {
                // Safe because builtin doesn't mutate builtins map
                std::mem::transmute::<&BuiltinFn, &BuiltinFn>(builtin)
            };
            return builtin_clone(self, args);
        }

        // 2. Check user-defined functions
        if let Some((params, body)) = self.functions.get(name).cloned() {
            let mut new_frame = StackFrame::new();

            for (i, (param_name, default_expr)) in params.iter().enumerate() {
                let val = if i < args.len() {
                    args[i].clone()
                } else if let Some(def) = default_expr {
                    self.eval_expr(def)?
                } else {
                    Value::Null
                };
                new_frame.locals.insert(param_name.clone(), val);
            }

            self.call_stack.push(new_frame);
            let result = self.execute(&body)?;
            self.call_stack.pop();

            return Ok(result.unwrap_or(Value::Null));
        }

        Err(format!("Call to undefined function {}()", name))
    }

    fn eval_binop(&self, op: &BinOpKind, left: Value, right: Value) -> Result<Value, String> {
        match op {
            BinOpKind::Add => match (&left, &right) {
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                _ => Ok(Value::Int(left.to_int_lossy() + right.to_int_lossy())),
            },
            BinOpKind::Sub => match (&left, &right) {
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                _ => Ok(Value::Int(left.to_int_lossy() - right.to_int_lossy())),
            },
            BinOpKind::Mul => match (&left, &right) {
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                _ => Ok(Value::Int(left.to_int_lossy() * right.to_int_lossy())),
            },
            BinOpKind::Div => match (&left, &right) {
                (Value::Float(a), Value::Float(b)) => {
                    if *b == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Float(a / b))
                    }
                }
                _ => {
                    let b = right.to_int_lossy();
                    if b == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Int(left.to_int_lossy() / b))
                    }
                }
            },
            BinOpKind::Concat => {
                let s = format!("{}{}", left.to_string_lossy(), right.to_string_lossy());
                Ok(Value::String(s))
            }
            BinOpKind::Eq => Ok(Value::Bool(left == right)),
            BinOpKind::Identical => Ok(Value::Bool(left == right)),
            BinOpKind::NotEq => Ok(Value::Bool(left != right)),
            BinOpKind::Lt => Ok(Value::Bool(left.to_int_lossy() < right.to_int_lossy())),
            BinOpKind::Gt => Ok(Value::Bool(left.to_int_lossy() > right.to_int_lossy())),
            BinOpKind::Lte => Ok(Value::Bool(left.to_int_lossy() <= right.to_int_lossy())),
            BinOpKind::Gte => Ok(Value::Bool(left.to_int_lossy() >= right.to_int_lossy())),
            BinOpKind::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinOpKind::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
        }
    }
}
