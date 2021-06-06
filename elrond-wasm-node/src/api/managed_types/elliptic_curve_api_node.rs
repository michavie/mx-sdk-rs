use super::ArwenBigUint;
use crate::String;
use elrond_wasm::api::EllipticCurveApi;
use elrond_wasm::types::BoxedBytes;
use elrond_wasm::*;

extern "C" {
	fn bigIntNew(value: i64) -> i32;

	fn addEC(
		xResultHandle: i32,
		yResultHandle: i32,
		ecHandle: i32,
		fstPointXHandle: i32,
		fstPointYHandle: i32,
		sndPointXHandle: i32,
		sndPointYHandle: i32,
	);

	fn doubleEC(
		xResultHandle: i32,
		yResultHandle: i32,
		ecHandle: i32,
		pointXHandle: i32,
		pointYHandle: i32,
	);

	fn isOnCurveEC(ecHandle: i32, pointXHandle: i32, pointYHandle: i32) -> i32;

	fn scalarMultEC(
		xResultHandle: i32,
		yResultHandle: i32,
		ecHandle: i32,
		pointXHandle: i32,
		pointYHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn scalarBaseMultEC(
		xResultHandle: i32,
		yResultHandle: i32,
		ecHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn marshalEC(xPairHandle: i32, yPairHandle: i32, ecHandle: i32, resultOffset: *mut u8) -> i32;

	fn marshalCompressedEC(
		xPairHandle: i32,
		yPairHandle: i32,
		ecHandle: i32,
		resultOffset: *mut u8,
	) -> i32;

	fn unmarshalEC(
		xPairHandle: i32,
		yPairHandle: i32,
		ecHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn unmarshalCompressedEC(
		xPairHandle: i32,
		yPairHandle: i32,
		ecHandle: i32,
		dataOffset: *const u8,
		length: i32,
	) -> i32;

	fn generateKeyEC(
		xPubKeyHandle: i32,
		yPubKeyHandle: i32,
		ecHandle: i32,
		resultOffset: *mut u8,
	) -> i32;

	fn ellipticCurveNew(
		fieldOrderHandle: i32,
		basePointOrderHandle: i32,
		eqConstantHandle: i32,
		xBasePointHandle: i32,
		yBasePointHandle: i32,
		sizeOfField: i32,
	) -> i32;

	fn ellipticCurveGetValues(
		ecHandle: i32,
		fieldOrderHandle: i32,
		basePointOrderHandle: i32,
		eqConstantHandle: i32,
		xBasePointHandle: i32,
		yBasePointHandle: i32,
	) -> i32;

	fn getCurveLengthEC(ecHandle: i32) -> i32;

	fn getCurveByteLengthEC(ecHandle: i32) -> i32;

	fn p224Ec() -> i32;

	fn p256Ec() -> i32;

	fn p384Ec() -> i32;

	fn p521Ec() -> i32;
}

pub struct ArwenEllipticCurve {
	pub handle: i32,
}

impl elrond_wasm::abi::TypeAbi for ArwenEllipticCurve {
	fn type_name() -> String {
		String::from("EllipticCurve")
	}
}

impl EllipticCurveApi for ArwenEllipticCurve {
	type BigUint = ArwenBigUint;

	fn new_elliptic_curve(
		field_order: Self::BigUint,
		base_point_order: Self::BigUint,
		eq_constant: Self::BigUint,
		x_base_point: Self::BigUint,
		y_base_point: Self::BigUint,
		size_of_field: u32,
	) -> Self {
		unsafe {
			let handle = ellipticCurveNew(
				field_order.handle,
				base_point_order.handle,
				eq_constant.handle,
				x_base_point.handle,
				y_base_point.handle,
				size_of_field as i32,
			);
			ArwenEllipticCurve { handle }
		}
	}

	fn get_values(
		&self,
	) -> (
		Self::BigUint,
		Self::BigUint,
		Self::BigUint,
		Self::BigUint,
		Self::BigUint,
		u32,
	) {
		unsafe {
			let field_order_handle = bigIntNew(0);
			let base_point_order_handle = bigIntNew(0);
			let eq_constant_handle = bigIntNew(0);
			let x_base_point_handle = bigIntNew(0);
			let y_base_point_handle = bigIntNew(0);
			let _handle = ellipticCurveGetValues(
				self.handle,
				field_order_handle,
				base_point_order_handle,
				eq_constant_handle,
				x_base_point_handle,
				y_base_point_handle,
			);
			(
				ArwenBigUint {
					handle: field_order_handle,
				},
				ArwenBigUint {
					handle: base_point_order_handle,
				},
				ArwenBigUint {
					handle: eq_constant_handle,
				},
				ArwenBigUint {
					handle: x_base_point_handle,
				},
				ArwenBigUint {
					handle: y_base_point_handle,
				},
				self.get_ec_length(),
			)
		}
	}

	fn p224_ec() -> Self {
		unsafe {
			let handle = p224Ec();
			ArwenEllipticCurve { handle }
		}
	}

	fn p256_ec() -> Self {
		unsafe {
			let handle = p256Ec();
			ArwenEllipticCurve { handle }
		}
	}

	fn p384_ec() -> Self {
		unsafe {
			let handle = p384Ec();
			ArwenEllipticCurve { handle }
		}
	}

	fn p521_ec() -> Self {
		unsafe {
			let handle = p521Ec();
			ArwenEllipticCurve { handle }
		}
	}

	fn get_ec_length(&self) -> u32 {
		unsafe { getCurveLengthEC(self.handle) as u32 }
	}

	fn add_ec(
		&self,
		x_first_point: Self::BigUint,
		y_first_point: Self::BigUint,
		x_second_point: Self::BigUint,
		y_second_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			addEC(
				x_result_handle,
				y_result_handle,
				self.handle,
				x_first_point.handle,
				y_first_point.handle,
				x_second_point.handle,
				y_second_point.handle,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn double_ec(
		&self,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			doubleEC(
				x_result_handle,
				y_result_handle,
				self.handle,
				x_point.handle,
				y_point.handle,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn is_on_curve_ec(&self, x_point: Self::BigUint, y_point: Self::BigUint) -> bool {
		unsafe { isOnCurveEC(self.handle, x_point.handle, y_point.handle) == 1 }
	}

	fn scalar_mult(
		&self,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			scalarMultEC(
				x_result_handle,
				y_result_handle,
				self.handle,
				x_point.handle,
				y_point.handle,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn scalar_base_mult(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		unsafe {
			let x_result_handle = bigIntNew(0);
			let y_result_handle = bigIntNew(0);
			scalarBaseMultEC(
				x_result_handle,
				y_result_handle,
				self.handle,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_result_handle,
				},
				ArwenBigUint {
					handle: y_result_handle,
				},
			)
		}
	}

	fn marshal_ec(&self, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes {
		unsafe {
			let curve_length = getCurveLengthEC(self.handle);
			let mut result = BoxedBytes::allocate(1 + 2 * (curve_length as usize + 7) / 8);
			marshalEC(
				x_pair.handle,
				y_pair.handle,
				self.handle,
				result.as_mut_ptr(),
			);
			result
		}
	}

	fn marshal_compressed_ec(&self, x_pair: Self::BigUint, y_pair: Self::BigUint) -> BoxedBytes {
		unsafe {
			let curve_length = getCurveLengthEC(self.handle);
			let mut result = BoxedBytes::allocate(1 + (curve_length as usize + 7) / 8);
			marshalCompressedEC(
				x_pair.handle,
				y_pair.handle,
				self.handle,
				result.as_mut_ptr(),
			);
			result
		}
	}

	fn unmarshal_ec(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		unsafe {
			let x_pair_handle = bigIntNew(0);
			let y_pair_handle = bigIntNew(0);
			unmarshalEC(
				x_pair_handle,
				y_pair_handle,
				self.handle,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_pair_handle,
				},
				ArwenBigUint {
					handle: y_pair_handle,
				},
			)
		}
	}

	fn unmarshal_compressed_ec(&self, data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		unsafe {
			let x_pair_handle = bigIntNew(0);
			let y_pair_handle = bigIntNew(0);
			unmarshalCompressedEC(
				x_pair_handle,
				y_pair_handle,
				self.handle,
				data.as_ptr(),
				data.len() as i32,
			);
			(
				ArwenBigUint {
					handle: x_pair_handle,
				},
				ArwenBigUint {
					handle: y_pair_handle,
				},
			)
		}
	}

	fn generate_key_ec(&self) -> (Self::BigUint, Self::BigUint, BoxedBytes) {
		unsafe {
			let x_pub_key_handle = bigIntNew(0);
			let y_pub_key_handle = bigIntNew(0);
			let priv_key_length = getCurveByteLengthEC(self.handle);
			let mut private_key = BoxedBytes::allocate(priv_key_length as usize);
			generateKeyEC(
				x_pub_key_handle,
				y_pub_key_handle,
				self.handle,
				private_key.as_mut_ptr(),
			);
			(
				ArwenBigUint {
					handle: x_pub_key_handle,
				},
				ArwenBigUint {
					handle: y_pub_key_handle,
				},
				private_key,
			)
		}
	}
}

use elrond_codec::*;

impl NestedEncode for ArwenEllipticCurve {
	fn dep_encode<O: NestedEncodeOutput>(
		&self,
		dest: &mut O,
	) -> core::result::Result<(), EncodeError> {
		let (field_order, base_point_order, eq_constant, x_base_point, y_base_point, size_of_field) =
			self.get_values();
		NestedEncode::dep_encode(&field_order, dest)?;
		NestedEncode::dep_encode(&base_point_order, dest)?;
		NestedEncode::dep_encode(&eq_constant, dest)?;
		NestedEncode::dep_encode(&x_base_point, dest)?;
		NestedEncode::dep_encode(&y_base_point, dest)?;
		NestedEncode::dep_encode(&size_of_field, dest)?;
		Ok(())
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		let (field_order, base_point_order, eq_constant, x_base_point, y_base_point, size_of_field) =
			self.get_values();
		NestedEncode::dep_encode_or_exit(&field_order, dest, c.clone(), exit);
		NestedEncode::dep_encode_or_exit(&base_point_order, dest, c.clone(), exit);
		NestedEncode::dep_encode_or_exit(&eq_constant, dest, c.clone(), exit);
		NestedEncode::dep_encode_or_exit(&x_base_point, dest, c.clone(), exit);
		NestedEncode::dep_encode_or_exit(&y_base_point, dest, c.clone(), exit);
		NestedEncode::dep_encode_or_exit(&size_of_field, dest, c, exit);
	}
}

impl TopEncode for ArwenEllipticCurve {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		top_encode_from_nested(self, output)
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		top_encode_from_nested_or_exit(self, output, c, exit);
	}
}

impl NestedDecode for ArwenEllipticCurve {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let field_order = <Self as EllipticCurveApi>::BigUint::dep_decode(input)?;
		let base_point_order = <Self as EllipticCurveApi>::BigUint::dep_decode(input)?;
		let eq_constant = <Self as EllipticCurveApi>::BigUint::dep_decode(input)?;
		let x_base_point = <Self as EllipticCurveApi>::BigUint::dep_decode(input)?;
		let y_base_point = <Self as EllipticCurveApi>::BigUint::dep_decode(input)?;
		let size_of_field = u32::dep_decode(input)?;
		Ok(ArwenEllipticCurve::new_elliptic_curve(
			field_order,
			base_point_order,
			eq_constant,
			x_base_point,
			y_base_point,
			size_of_field,
		))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let field_order =
			<Self as EllipticCurveApi>::BigUint::dep_decode_or_exit(input, c.clone(), exit);
		let base_point_order =
			<Self as EllipticCurveApi>::BigUint::dep_decode_or_exit(input, c.clone(), exit);
		let eq_constant =
			<Self as EllipticCurveApi>::BigUint::dep_decode_or_exit(input, c.clone(), exit);
		let x_base_point =
			<Self as EllipticCurveApi>::BigUint::dep_decode_or_exit(input, c.clone(), exit);
		let y_base_point =
			<Self as EllipticCurveApi>::BigUint::dep_decode_or_exit(input, c.clone(), exit);
		let size_of_field = u32::dep_decode_or_exit(input, c, exit);
		ArwenEllipticCurve::new_elliptic_curve(
			field_order,
			base_point_order,
			eq_constant,
			x_base_point,
			y_base_point,
			size_of_field,
		)
	}
}

impl TopDecode for ArwenEllipticCurve {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		top_decode_from_nested(input)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		top_decode_from_nested_or_exit(input, c, exit)
	}
}
