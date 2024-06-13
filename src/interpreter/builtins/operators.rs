use std::collections::HashMap;

use crate::interpreter::{
    error::FruError,
    identifier::{Identifier as Id, OperatorIdentifier as OpId},
    value::fru_value::FruValue,
    value::operator::AnyOperator,
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

macro_rules! operator_group {
    ($ident1:expr, $ident2:expr, [$(($op:ident, $fn_name:ident)),*]) => {
        [
            $(
                (
                    OpId::new(Id::$op(), $ident1, $ident2),
                    AnyOperator::BuiltinOperator($fn_name),
                )
            ),*
        ]
    };
}

pub fn builtin_operators() -> HashMap<OpId, AnyOperator> {
    let mut res = HashMap::from(operator_group!(
        Id::for_number(),
        Id::for_number(),
        [
            (for_plus, num_plus_num),
            (for_minus, num_minus_num),
            (for_multiply, num_mul_num),
            (for_divide, num_div_num),
            (for_mod, num_mod_num),
            (for_pow, num_pow_num),
            (for_less, num_less_num),
            (for_less_eq, num_less_eq_num),
            (for_greater, num_greater_num),
            (for_greater_eq, num_greater_eq_num),
            (for_eq, num_eq_num),
            (for_not_eq, num_not_eq_num)
        ]
    ));

    res.extend(operator_group!(
        Id::for_bool(),
        Id::for_bool(),
        [(for_and, bool_and_bool), (for_or, bool_or_bool)]
    ));

    res.extend(operator_group!(
        Id::for_string(),
        Id::for_string(),
        [
            (for_combine, string_concat),
            (for_less, string_less_string),
            (for_less_eq, string_less_eq_string),
            (for_greater, string_greater_string),
            (for_greater_eq, string_greater_eq_string),
            (for_eq, string_eq_string),
            (for_not_eq, string_not_eq_string)
        ]
    ));

    res.extend([
        (
            OpId::new(Id::for_multiply(), Id::for_string(), Id::for_number()),
            AnyOperator::BuiltinOperator(string_mul_num),
        ),
        (
            OpId::new(Id::for_multiply(), Id::for_number(), Id::for_string()),
            AnyOperator::BuiltinOperator(num_mul_string),
        ),
    ]);

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
builtin_operator!(string_less_string, String, String, Bool, <);
builtin_operator!(string_less_eq_string, String, String, Bool, <=);
builtin_operator!(string_greater_string, String, String, Bool, >);
builtin_operator!(string_greater_eq_string, String, String, Bool, >=);
builtin_operator!(string_eq_string, String, String, Bool, ==);
builtin_operator!(string_not_eq_string, String, String, Bool, !=);
fn string_concat(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    if let (FruValue::String(l), FruValue::String(r)) = (left, right) {
        return Ok(FruValue::String(l + &*r));
    }

    unreachable!();
}

fn string_mul_num(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    if let (FruValue::String(l), FruValue::Number(r)) = (left, right) {
        if r.fract() != 0.0 || r < 0.0 {
            return FruError::new_res("String * number must be a positive integer");
        }

        return Ok(FruValue::String(l.repeat(r as usize)));
    }

    unreachable!();
}

fn num_mul_string(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    string_mul_num(right, left)
}
