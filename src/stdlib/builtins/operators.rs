use crate::{
    interpreter::{
        error::FruError,
        identifier::{id, OperatorIdentifier},
        value::{
            fru_value::FruValue, native_object::cast_object, native_object::NativeObject,
            operator::AnyOperator,
        },
    },
    stdlib::builtins::{
        builtin_bool_type::BuiltinBoolType, builtin_number_type::BuiltinNumberType,
        builtin_string_instance::BuiltinStringInstance, builtin_string_type::BuiltinStringType,
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

macro_rules! builtin_operator_string {
    ($Name:ident, $Res:expr, $OP:tt) => {
        fn $Name(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
            let l = &cast_object::<BuiltinStringInstance>(&left).unwrap().value;
            let r = &cast_object::<BuiltinStringInstance>(&right).unwrap().value;
            Ok($Res(l $OP r))
        }
    };
}

macro_rules! operator_group {
    ($left:expr, $right:expr, [$(($op:expr, $fn_name:ident),)*]) => {
        $(
            $left.set_operator(
                OperatorIdentifier::new($op, $right.get_uid()),
                AnyOperator::BuiltinOperator($fn_name),
            ).unwrap();
        )*
    };
}

#[ctor::ctor]
fn ct() {
    operator_group!(
        BuiltinNumberType::get_value(),
        BuiltinNumberType::get_value(),
        [
            (id::LESS, num_less_num),
            (id::LESS_EQ, num_less_eq_num),
            (id::GREATER, num_greater_num),
            (id::GREATER_EQ, num_greater_eq_num),
            (id::EQ, num_eq_num),
            (id::NOT_EQ, num_not_eq_num),
            (id::PLUS, num_plus_num),
            (id::MINUS, num_minus_num),
            (id::MULTIPLY, num_mul_num),
            (id::DIVIDE, num_div_num),
            (id::MOD, num_mod_num),
            (id::POW, num_pow_num),
        ]
    );
    operator_group!(
        BuiltinBoolType::get_value(),
        BuiltinBoolType::get_value(),
        [(id::AND, bool_and_bool), (id::OR, bool_or_bool),]
    );

    operator_group!(
        BuiltinStringType::get_value(),
        BuiltinStringType::get_value(),
        [
            (id::LESS, string_less_string),
            (id::LESS_EQ, string_less_eq_string),
            (id::GREATER, string_greater_string),
            (id::GREATER_EQ, string_greater_eq_string),
            (id::EQ, string_eq_string),
            (id::NOT_EQ, string_not_eq_string),
            (id::COMBINE, string_concat),
        ]
    );

    operator_group!(
        BuiltinNumberType::get_value(),
        BuiltinStringType::get_value(),
        [(id::MULTIPLY, num_mul_string),]
    );

    operator_group!(
        BuiltinStringType::get_value(),
        BuiltinNumberType::get_value(),
        [(id::MULTIPLY, string_mul_num),]
    );
}

// number cmp
builtin_operator!(num_less_num, Number, Number, Bool, <);
builtin_operator!(num_less_eq_num, Number, Number, Bool, <=);
builtin_operator!(num_greater_num, Number, Number, Bool, >);
builtin_operator!(num_greater_eq_num, Number, Number, Bool, >=);
builtin_operator!(num_eq_num, Number, Number, Bool, ==);
builtin_operator!(num_not_eq_num, Number, Number, Bool, !=);

// number arithmetic
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

// bool
builtin_operator!(bool_or_bool, Bool, Bool, Bool, ||);
builtin_operator!(bool_and_bool, Bool, Bool, Bool, &&);

// string cmp
builtin_operator_string!(string_less_string, FruValue::Bool, <);
builtin_operator_string!(string_less_eq_string, FruValue::Bool, <=);
builtin_operator_string!(string_greater_string, FruValue::Bool, >);
builtin_operator_string!(string_greater_eq_string, FruValue::Bool, >=);
builtin_operator_string!(string_eq_string, FruValue::Bool, ==);
builtin_operator_string!(string_not_eq_string, FruValue::Bool, !=);

// string arithmetic
fn string_concat(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    let l = &cast_object::<BuiltinStringInstance>(&left).unwrap().value;
    let r = &cast_object::<BuiltinStringInstance>(&right).unwrap().value;
    Ok(NativeObject::new_value(BuiltinStringInstance::new(
        l.to_owned() + r,
    )))
}

fn string_mul_num(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    if let FruValue::Number(r) = right {
        if r.fract() != 0.0 || r < 0.0 {
            return FruError::new_res("String * number must be a positive integer");
        }

        let l = &cast_object::<BuiltinStringInstance>(&left).unwrap().value;
        let r = r as usize;

        return Ok(NativeObject::new_value(BuiltinStringInstance::new(
            l.repeat(r),
        )));
    }

    unreachable!();
}

fn num_mul_string(left: FruValue, right: FruValue) -> Result<FruValue, FruError> {
    string_mul_num(right, left)
}
