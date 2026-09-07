#![allow(clippy::module_inception)]
use rustpython_vm::pymodule;

#[pymodule]
pub mod unittest {
    use rustpython_vm::function::OptionalArg;
    use rustpython_vm::{pyclass, AsObject, PyObjectRef, PyPayload, PyResult, VirtualMachine};
    use std::ops::Not;
    use rustpython_vm::function::OptionalArg::Present;

    #[pyattr]
    #[pyclass(name)]
    #[derive(Debug, PyPayload)]
    pub struct TestCase {}

    #[pyclass(flags(BASETYPE))] // Enables inheritance
    #[opendut_viper_pygen::pygen]
    impl TestCase {

        /// Test that `a` and `b` are equal. If the values do not compare equal, the test will fail.
        #[pymethod(name = "assertEquals")]
        #[viper(name = "assertEquals")]
        fn assert_equals(
            _this: PyObjectRef,
            a: PyObjectRef,
            b: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let is_equals = vm.call_method(&a, "__eq__", vec![b.clone()])?;

            Self::ensure_comparable(&is_equals, &message, vm)?;

            let a_value = a.repr(vm)?.to_string();
            let b_value = b.repr(vm)?.to_string();

            if is_equals.is_true(vm)? {
                Ok(())
            } else {
                let error_message = format!("ASSERTION FAILED: {a_value} != {b_value}{}", Self::format_message(message));
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `a` and `b` are not equal. If the values do compare equal, the test will fail.
        #[pymethod(name = "assertNotEquals")]
        #[viper(name = "assertNotEquals")]
        fn assert_not_equals(
            _this: PyObjectRef,
            a: PyObjectRef,
            b: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let is_equals = vm.call_method(&a, "__eq__", vec![b.clone()])?;

            Self::ensure_comparable(&is_equals, &message, vm)?;

            let a_value = a.repr(vm)?.to_string();
            let b_value = b.repr(vm)?.to_string();

            if is_equals.is_true(vm)?.not() {
                Ok(())
            } else {
                let error_message = format!("ASSERTION FAILED: {a_value} == {b_value}{}", Self::format_message(message));
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that given `expression` is `True`.
        #[pymethod(name = "assertTrue")]
        #[viper(name = "assertTrue")]
        fn assert_true(
            _this: PyObjectRef,
            expression: bool,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let error_message = format!("ASSERTION FAILED: {expression} is not true{}", Self::format_message(message));

            if expression {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that given `expression` is `False`.
        #[pymethod(name = "assertFalse")]
        #[viper(name = "assertFalse")]
        fn assert_false(
            _this: PyObjectRef,
            expression: bool,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {

            let error_message = format!("ASSERTION FAILED: {expression} is not false{}", Self::format_message(message));

            if expression.not() {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `a` and `b` are the same object.
        #[pymethod(name = "assertIs")]
        #[viper(name = "assertIs")]
        fn assert_is(
            _this: PyObjectRef,
            a: PyObjectRef,
            b: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let a_value = a.repr(vm)?.to_string();
            let b_value = b.repr(vm)?.to_string();

            let error_message = format!("ASSERTION FAILED: {a_value} is not {b_value}{}", Self::format_message(message));

            if a.is(&b) {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `a` and `b` are different objects.
        #[pymethod(name = "assertIsNot")]
        #[viper(name = "assertIsNot")]
        fn assert_is_not(
            _this: PyObjectRef,
            a: PyObjectRef,
            b: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let a_value = a.repr(vm)?.to_string();
            let error_message = format!("ASSERTION FAILED: Unexpectedly identical: {a_value}{}",Self::format_message(message));

            if a.is(&b).not() {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `object` is None.
        #[pymethod(name = "assertIsNone")]
        #[viper(name = "assertIsNone")]
        fn assert_is_none(
            _this: PyObjectRef,
            object: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let object_value = object.repr(vm)?.to_string();

            let error_message = format!("ASSERTION FAILED: {object_value} is not None{}", Self::format_message(message));

            if vm.is_none(&object) {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `object` is not None.
        #[pymethod(name = "assertIsNotNone")]
        #[viper(name = "assertIsNotNone")]
        fn assert_is_not_none(
            _this: PyObjectRef,
            object: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let error_message = format!("ASSERTION FAILED: Unexpectedly None{}", Self::format_message(message));

            if vm.is_none(&object).not() {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `element` is in `container`.
        #[pymethod(name = "assertIn")]
        #[viper(name = "assertIn")]
        fn assert_in(
            _this: PyObjectRef,
            element: PyObjectRef,
            container: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let result = vm.call_method(&container, "__contains__", vec![element.clone()])?;

            let element_value = element.repr(vm)?.to_string();
            let container_value = container.repr(vm)?.to_string();

            let error_message = format!("ASSERTION FAILED: {element_value} not found in {container_value}{}", Self::format_message(message));

            if result.is_true(vm)? {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `element` is not in `container`.
        #[pymethod(name = "assertNotIn")]
        #[viper(name = "assertNotIn")]
        fn assert_not_in(
            _this: PyObjectRef,
            element: PyObjectRef,
            container: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let result = vm.call_method(&container, "__contains__", vec![element.clone()])?;

            let element_value = element.repr(vm)?.to_string();
            let container_value = container.repr(vm)?.to_string();


            let error_message = format!("ASSERTION FAILED: {element_value} unexpectedly found in {container_value}{}", Self::format_message(message));

            if result.is_true(vm)?.not() {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `object` is an instance of `cls`.
        #[pymethod(name = "assertIsInstance")]
        #[viper(name = "assertIsInstance")]
        fn assert_is_instance(
            _this: PyObjectRef,
            object: PyObjectRef,
            cls: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let is_instance = object.is_instance(&cls, vm)?;
            let object_value = object.repr(vm)?.to_string();
            let cls_value = cls.repr(vm)?.to_string();
            let error_message = format!("ASSERTION FAILED: {object_value} is not an instance of {cls_value}{}", Self::format_message(message));

            if is_instance {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `object` is not an instance of `cls`.
        #[pymethod(name = "assertIsNotInstance")]
        #[viper(name = "assertIsNotInstance")]
        fn assert_is_not_instance(
            _this: PyObjectRef,
            object: PyObjectRef,
            cls: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let is_instance = object.is_instance(&cls, vm)?;
            let object_value = object.repr(vm)?.to_string();
            let cls_value = cls.repr(vm)?.to_string();
            let error_message = format!("ASSERTION FAILED: {object_value} is an instance of {cls_value}{}", Self::format_message(message));

            if is_instance.not() {
                Ok(())
            } else {
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `left` is greater than `right`, otherwise the test will fail.
        #[pymethod(name = "assertGreater")]
        #[viper(name = "assertGreater")]
        fn assert_greater(
            _this: PyObjectRef,
            left: PyObjectRef,
            right: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let result = vm.call_method(&left, "__gt__", vec![right.clone()])?;

            Self::ensure_comparable(&result, &message, vm)?;

            let left_value = left.repr(vm)?.to_string();
            let right_value = right.repr(vm)?.to_string();

            if result.is_true(vm)? {
                Ok(())
            } else {
                let error_message = format!("ASSERTION FAILED: {left_value} not greater than {right_value}{}", Self::format_message(message));
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `left` is less than `right`, otherwise the test will fail.
        #[pymethod(name = "assertLess")]
        #[viper(name = "assertLess")]
        fn assert_less(
            _this: PyObjectRef,
            left: PyObjectRef,
            right: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let result = vm.call_method(&left, "__lt__", vec![right.clone()])?;

            Self::ensure_comparable(&result, &message, vm)?;

            let left_value = left.repr(vm)?.to_string();
            let right_value = right.repr(vm)?.to_string();

            if result.is_true(vm)? {
                Ok(())
            } else {
                let error_message = format!("ASSERTION FAILED: {left_value} not less than {right_value}{}", Self::format_message(message));
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `left` is greater than or equal to `right`, otherwise the test will fail.
        #[pymethod(name = "assertGreaterOrEqual")]
        #[viper(name = "assertGreaterOrEqual")]
        fn assert_greater_or_equal(
            _this: PyObjectRef,
            left: PyObjectRef,
            right: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let result = vm.call_method(&left, "__ge__", vec![right.clone()])?;

            Self::ensure_comparable(&result, &message, vm)?;

            let left_value = left.repr(vm)?.to_string();
            let right_value = right.repr(vm)?.to_string();

            if result.is_true(vm)? {
                Ok(())
            } else {
                let error_message = format!("ASSERTION FAILED: {left_value} not greater than or equal to {right_value}{}", Self::format_message(message));
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Test that `left` is less than or equal to `right`, otherwise the test will fail.
        #[pymethod(name = "assertLessOrEqual")]
        #[viper(name = "assertLessOrEqual")]
        fn assert_less_or_equal(
            _this: PyObjectRef,
            left: PyObjectRef,
            right: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let result = vm.call_method(&left, "__le__", vec![right.clone()])?;

            Self::ensure_comparable(&result, &message, vm)?;

            let left_value = left.repr(vm)?.to_string();
            let right_value = right.repr(vm)?.to_string();

            if result.is_true(vm)? {
                Ok(())
            } else {
                let error_message = format!("ASSERTION FAILED: {left_value} not less than or equal to {right_value}{}", Self::format_message(message));
                Err(vm.new_runtime_error(error_message))
            }
        }

        /// Signals a test failure unconditionally.
        #[pymethod]
        fn fail(
            _this: PyObjectRef,
            #[viper(default = "")] message: OptionalArg<String>,
            #[viper(skip)] vm: &VirtualMachine
        ) -> PyResult<()> {
            let message = message.unwrap_or_default();
            Err(vm.new_runtime_error(message))
        }

        fn ensure_comparable(
            result: &PyObjectRef,
            message: &OptionalArg<String>,
            vm: &VirtualMachine,
        ) -> PyResult<()> {

            if result.is(&vm.ctx.not_implemented) {
                let error_message = message
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| String::from("ASSERTION FAILED: Objects are not comparable"));

                Err(vm.new_runtime_error(error_message))
            } else {
                Ok(())
            }
        }

        fn format_message(message: OptionalArg<String>) -> String {
            if let Present(message) = message {
                format!(": {}", message)
            } else {
                String::new()
            }

        }
    }
}
