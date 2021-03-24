fn main() {
    unsafe_rust::unsafe_fn();
    operator_overload::it_works();
    advanced_type::it_works();
    advanced_fn::it_works();
}

mod unsafe_rust;
mod operator_overload;
mod advanced_type;
mod advanced_fn;