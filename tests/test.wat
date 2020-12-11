(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add)
  (memory $mem 1)
  (global $g i32 (i32.const 42))
  (table $functable 2 funcref)
  (export "add" (func $add))
  (export "memory" (memory $mem))
  (export "meaning" (global $g))
  (export "functable" (table $functable))
)