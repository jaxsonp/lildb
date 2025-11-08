use super::*;

pub const createFunction: FunctionDef = FunctionDef {
	name: "create",
	positional_args: &[],
	object_type: Type::Object,
	return_type: Type::None,
};

pub const ensureExistsFunction: FunctionDef = FunctionDef {
	name: "ensure_exists",
	positional_args: &[],
	object_type: Type::Object,
	return_type: Type::Object,
};

pub const deleteFunction: FunctionDef = FunctionDef {
	name: "delete",
	positional_args: &[],
	object_type: Type::Object,
	return_type: Type::Object,
};
