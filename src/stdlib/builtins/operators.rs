use std::collections::HashMap;

use crate::{
    interpreter::{
        error::FruError,
        identifier::{id, OperatorIdentifier},
        value::{fru_value::FruValue, native_object::INativeObject, operator::AnyOperator},
    },
    stdlib::builtins::{
        builtin_bool_type::BuiltinBoolType, builtin_number_type::BuiltinNumberType,
    },
};

macro_rules! builtin_operator {
    ($Name:ident, $L:ident, $R:ident, $Res:ident, $OP:tt) => {
        fn $Name(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
            if let (FruValue::$L(l), FruValue::$R(r)) = (left, right) {
                return Ok(FruValue::$Res(l $OP r));
            }

            unreachable!();
        }
    };
}

// macro_rules! builtin_operator_string {
//     ($Name:ident, $Res:expr, $OP:tt) => {
//         fn $Name(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
//             let l = &cast_object::<BuiltinStringInstance>(&left).unwrap().value;
//             let r = &cast_object::<BuiltinStringInstance>(&right).unwrap().value;
//             Ok($Res(l $OP r))
//         }
//     };
// }

macro_rules! operator_group {
    ($left_ident:expr, $right_ident:expr, [$(($op:ident, $fn_name:ident)),*]) => {
        [
            $(
                (
                    OperatorIdentifier::new(id::$op, $left_ident, $right_ident),
                    AnyOperator::BuiltinOperator($fn_name),
                )
            ),*
        ]
    };
}

pub fn builtin_operators() -> HashMap<OperatorIdentifier, AnyOperator> {
    let mut res = HashMap::new();

    res.extend(operator_group!(
        BuiltinNumberType.get_uid(),
        BuiltinNumberType.get_uid(),
        [
            (PLUS, num_plus_num),
            (MINUS, num_minus_num),
            (MULTIPLY, num_mul_num),
            (DIVIDE, num_div_num),
            (MOD, num_mod_num),
            (POW, num_pow_num),
            (LESS, num_less_num),
            (LESS_EQ, num_less_eq_num),
            (GREATER, num_greater_num),
            (GREATER_EQ, num_greater_eq_num),
            (EQ, num_eq_num),
            (NOT_EQ, num_not_eq_num)
        ]
    ));

    res.extend(operator_group!(
        BuiltinBoolType.get_uid(),
        BuiltinBoolType.get_uid(),
        [(AND, bool_and_bool), (OR, bool_or_bool)]
    ));

    // res.extend(operator_group!(
    //     BuiltinStringType.get_uid(),
    //     BuiltinStringType.get_uid(),
    //     [
    //         (COMBINE, string_concat),
    //         (LESS, string_less_string),
    //         (LESS_EQ, string_less_eq_string),
    //         (GREATER, string_greater_string),
    //         (GREATER_EQ, string_greater_eq_string),
    //         (EQ, string_eq_string),
    //         (NOT_EQ, string_not_eq_string)
    //     ]
    // ));
    //
    // res.extend([
    //     (
    //         OperatorIdentifier::new(
    //             id::MULTIPLY,
    //             BuiltinStringType.get_uid(),
    //             BuiltinNumberType.get_uid(),
    //         ),
    //         AnyOperator::BuiltinOperator(string_mul_num),
    //     ),
    //     (
    //         OperatorIdentifier::new(
    //             id::MULTIPLY,
    //             BuiltinNumberType.get_uid(),
    //             BuiltinStringType.get_uid(),
    //         ),
    //         AnyOperator::BuiltinOperator(num_mul_string),
    //     ),
    // ]);

    res
}

// number
builtin_operator!(num_plus_num, Number, Number, Number, +);
builtin_operator!(num_minus_num, Number, Number, Number, -);
builtin_operator!(num_mul_num, Number, Number, Number, *);
fn num_div_num(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    if let (FruValue::Number(l), FruValue::Number(r)) = (left, right) {
        if r == 0.0 {
            return FruError::new_res("division by zero");
        }
        return Ok(FruValue::Number(l / r));
    }

    unreachable!();
}

fn num_mod_num(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    if let (FruValue::Number(l), FruValue::Number(r)) = (left, right) {
        if r == 0.0 {
            return FruError::new_res("division by zero");
        }
        return Ok(FruValue::Number(l.rem_euclid(r)));
    }

    unreachable!();
}

fn num_pow_num(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    if let (FruValue::Number(l), FruValue::Number(r)) = (left, right) {
        return Ok(FruValue::Number(l.powf(r)));
    }

    unreachable!();
}
builtin_operator!(num_less_num, Number, Number, Bool, <);
builtin_operator!(num_less_eq_num, Number, Number, Bool, <=);
builtin_operator!(num_greater_num, Number, Number, Bool, >);
builtin_operator!(num_greater_eq_num, Number, Number, Bool, >=);
builtin_operator!(num_eq_num, Number, Number, Bool, ==);
builtin_operator!(num_not_eq_num, Number, Number, Bool, !=);

// bool
builtin_operator!(bool_or_bool, Bool, Bool, Bool, ||);
builtin_operator!(bool_and_bool, Bool, Bool, Bool, &&);

// string
// builtin_operator_string!(string_less_string, FruValue::Bool, <);
// builtin_operator_string!(string_less_eq_string, FruValue::Bool, <=);
// builtin_operator_string!(string_greater_string, FruValue::Bool, >);
// builtin_operator_string!(string_greater_eq_string, FruValue::Bool, >=);
// builtin_operator_string!(string_eq_string, FruValue::Bool, ==);
// builtin_operator_string!(string_not_eq_string, FruValue::Bool, !=);
//
// fn string_concat(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
//     let l = &cast_object::<BuiltinStringInstance>(&left).unwrap().value;
//     let r = &cast_object::<BuiltinStringInstance>(&right).unwrap().value;
//     Ok(NativeObject::new_value(BuiltinStringInstance::new(
//         l.to_owned() + r,
//     )))
// }
//
// fn string_mul_num(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
//     if let FruValue::Number(r) = right {
//         if r.fract() != 0.0 || r < 0.0 {
//             return FruError::new_res("String * number must be a positive integer");
//         }
//
//         let l = &cast_object::<BuiltinStringInstance>(&left).unwrap().value;
//         let r = r as usize;
//
//         return Ok(NativeObject::new_value(BuiltinStringInstance::new(
//             l.repeat(r),
//         )));
//     }
//
//     unreachable!();
// }
//
// fn num_mul_string(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
//     string_mul_num(right, left)
// }
