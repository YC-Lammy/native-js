use std::marker::PhantomData;

use types::{Auto, FunctionType, IntoMarkerType, Smart, ValueIndex};

use crate::function::{BlockDesc, Function};
use crate::mir::{FCond, ICond, Ordering, MIR};
use crate::types::simd::{LaneCount, SupportedLaneCount};
use crate::types::{
    Aggregate, FieldedMarkerType, FloatMarkerType, FloatMathMarkerType, IntMarkerType,
    IntMathMarkerType, Interface, IntoFloatMarkerType, IntoIntMarkerType, IntoScalarMarkerType,
    MarkerType, MathMarkerType, Pointer, ScalarMarkerType, Type, I32, I8, SIMD,
};
use crate::util::{AggregateID, BlockID, FunctionID, Ident, InterfaceID, StackSlotID};
pub use crate::Value;
use crate::{types, Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block<'func> {
    id: BlockID,
    _mark: PhantomData<&'func ()>,
}

#[derive(Clone)]
pub struct StackSlot<'ctx, 'func, T: MarkerType<'ctx>> {
    id: StackSlotID,
    ty: T,
    _mark: PhantomData<(&'func (), &'ctx ())>,
}

pub struct Builder<'ctx, 'func>
where
    'ctx: 'func,
{
    ctx: &'ctx mut Context,
    func: &'func mut Function<'ctx>,
    current_block: &'func mut BlockDesc<'ctx>,
}

impl<'ctx, 'func> Builder<'ctx, 'func>
where
    'ctx: 'func,
{
    pub fn create_block(&mut self) -> Block<'func> {
        let id = BlockID::new();
        self.func.blocks.push(BlockDesc::new(id, &[]));

        return Block {
            id,
            _mark: PhantomData,
        };
    }

    pub fn switch_to_block(&mut self, block: Block<'func>) {
        let block:&mut BlockDesc = self.func
                    .blocks
                    .iter_mut()
                    .rev()
                    .find(|b| b.id == block.id)
                    .expect("Trying to get instruction builder without declaring block");

        self.current_block = unsafe{core::mem::transmute(block)};
    }

    pub fn inst<'builder>(&'builder mut self) -> InstBuilder<'ctx, 'func, 'builder>{
        InstBuilder {
            ctx: self.ctx,
            func: self.func,
            block: self.current_block,
            _mark: PhantomData
        }
    }
}
pub struct InstBuilder<'ctx, 'func, 'builder>
where
    'ctx: 'func, 'func: 'builder
{
    ctx: &'builder mut Context,
    func: &'builder mut Function<'ctx>,
    block: &'builder mut crate::function::BlockDesc<'ctx>,
    _mark: PhantomData<&'func ()>
}

impl<'ctx, 'func, 'builder> InstBuilder<'ctx, 'func, 'builder>
where
    'ctx: 'func,
{
    /// an integer constant
    pub fn iconst<I: IntoIntMarkerType>(&mut self, value: I) -> Value<'ctx, 'func, I::Marker> {
        let id = self.block.new_id(I::Marker::default().to_type());
        self.block.inst.push(MIR::Iconst(value.to_i128(), id));

        return Value {
            id: id,
            ty: I::Marker::default(),
            _mark: PhantomData,
        };
    }

    /// a floating point constant
    pub fn fconst<F: IntoFloatMarkerType>(&mut self, value: F) -> Value<'ctx, 'func, F::Marker> {
        let id = self.block.new_id(F::Marker::default().to_type());

        if core::mem::size_of::<F>() == 8 {
            let v = unsafe { *(&value as *const F as *const f64) };
            self.block.inst.push(MIR::F64const(v, id));
        } else {
            debug_assert!(core::mem::size_of::<F>() == 4);
            let v = unsafe { *(&value as *const F as *const f32) };
            self.block.inst.push(MIR::F32const(v, id));
        }
        return Value {
            id,
            ty: F::Marker::default(),
            _mark: PhantomData,
        };
    }

    /// an simd constant
    pub fn vconst<T: IntoScalarMarkerType, const N: usize>(
        &mut self,
        values: [T; N],
    ) -> Value<'ctx, 'func, SIMD<T::Marker, N>>
    where
        crate::types::simd::LaneCount<N>: crate::types::simd::SupportedLaneCount,
    {
        let id = self.block.new_id(SIMD::<T::Marker, N>::default().to_type());
        unsafe {
            let layout = alloc::alloc::Layout::for_value(&values);
            let ptr = alloc::alloc::alloc(layout);
            core::ptr::copy_nonoverlapping(
                &values as *const [T; N] as *const u8,
                ptr,
                layout.size(),
            );
            let slice = core::slice::from_raw_parts_mut(ptr, layout.size());
            let data = Box::from_raw(slice);
            self.block.inst.push(MIR::Vconst(data, id));

            return Value {
                id,
                ty: SIMD::<T::Marker, N>::default(),
                _mark: PhantomData,
            };
        };
    }

    pub fn neg<T: MathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Neg(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }

    pub fn abs<T: MathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Abs(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }

    pub fn add<T: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, T>,
        b: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(a.ty.to_type());

        self.block.inst.push(MIR::Add(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn sub<T: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, T>,
        b: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Sub(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn mul<T: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, T>,
        b: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(a.ty.to_type());

        self.block.inst.push(MIR::Mul(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn exp<T: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, T>,
        b: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Exp(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn rem<T: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, T>,
        b: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Rem(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn div<T: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, T>,
        b: Value<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Div(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn shl<I: IntMathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::IShl(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn shr<I: IntMathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::IShr(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitand<I: IntMathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Bitand(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitor<I: IntMathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::BitOr(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitxor<I: IntMathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Bitxor(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitnot<I: IntMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Bitnot(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitrev<I: IntMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Bitrev(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitswap<I: IntMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Bitswap(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn count_ones<I: IntMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::BitOnes(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn leading_zeros<I: IntMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::BitLeadingZeros(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn trailing_zeros<I: IntMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::BitTrailingZeros(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitcast<T: MarkerType<'ctx>, U: MarkerType<'ctx>>(
        &mut self,
        value: Value<'ctx, 'func, T>,
        ty: U,
    ) -> Value<'ctx, 'func, U> {
        let id = self.block.new_id(ty.to_type());
        self.block.inst.push(MIR::Bitnot(value.id, id));

        return Value {
            id,
            ty: ty,
            _mark: PhantomData,
        };
    }

    pub fn icmp<I: IntMathMarkerType>(
        &mut self,
        cond: ICond,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Icmp(cond, a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn fcmp<F: FloatMathMarkerType>(
        &mut self,
        cond: FCond,
        a: Value<'ctx, 'func, F>,
        b: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Fcmp(cond, a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }

    pub fn min<I: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Min(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn max<I: MathMarkerType>(
        &mut self,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Max(a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn select<I: IntMathMarkerType>(
        &mut self,
        test: Value<'ctx, 'func, I>,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block.inst.push(MIR::Select(test.id, a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }
    pub fn bitselect<I: IntMathMarkerType>(
        &mut self,
        test: Value<'ctx, 'func, I>,
        a: Value<'ctx, 'func, I>,
        b: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, I> {
        let id = self.block.new_id(a.ty.to_type());
        self.block
            .inst
            .push(MIR::BitSelect(test.id, a.id, b.id, id));

        return Value {
            id,
            ty: a.ty,
            _mark: PhantomData,
        };
    }

    pub fn sqrt<F: FloatMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Sqrt(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn sin<F: FloatMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Sin(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn cos<F: FloatMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Cos(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn powi<F: FloatMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
        exponent: Value<'ctx, 'func, I32>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Powi(value.id, exponent.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn powf<F: FloatMathMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
        exponent: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Powf(value.id, exponent.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn floor<F: FloatMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Floor(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn ceil<F: FloatMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Ceil(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn round<F: FloatMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, F> {
        let id = self.block.new_id(value.ty.to_type());
        self.block.inst.push(MIR::Round(value.id, id));

        return Value {
            id,
            ty: value.ty,
            _mark: PhantomData,
        };
    }
    pub fn int_to_float<F: IntoFloatMarkerType, I: IntMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, F::Marker> {
        let id = self.block.new_id(F::Marker::default().to_type());
        self.block.inst.push(MIR::IntToFloat(value.id, id));

        return Value {
            id,
            ty: F::Marker::default(),
            _mark: PhantomData,
        };
    }
    pub fn float_to_int<I: IntoIntMarkerType, F: FloatMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, I::Marker> {
        let id = self.block.new_id(I::Marker::default().to_type());
        self.block.inst.push(MIR::FloatToInt(value.id, id));

        return Value {
            id,
            ty: I::Marker::default(),
            _mark: PhantomData,
        };
    }
    pub fn int_cast<U: IntoIntMarkerType, I: IntMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, I>,
    ) -> Value<'ctx, 'func, U::Marker> {
        let id = self.block.new_id(U::Marker::default().to_type());
        self.block.inst.push(MIR::IntCast(value.id, id));

        return Value {
            id,
            ty: U::Marker::default(),
            _mark: PhantomData,
        };
    }
    pub fn float_cast<U: IntoFloatMarkerType, F: FloatMarkerType>(
        &mut self,
        value: Value<'ctx, 'func, F>,
    ) -> Value<'ctx, 'func, U::Marker> {
        let id = self.block.new_id(U::Marker::default().to_type());
        self.block.inst.push(MIR::FloatCast(value.id, id));

        return Value {
            id,
            ty: U::Marker::default(),
            _mark: PhantomData,
        };
    }
    pub fn extract_element<T: ScalarMarkerType, const N: usize>(
        &mut self,
        vector: Value<'ctx, 'func, SIMD<T, N>>,
        index: u8,
    ) -> Value<'ctx, 'func, T>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        if index as usize >= N {
            panic!("index larger then lanes")
        }
        let id = self.block.new_id(T::default().to_type());
        self.block
            .inst
            .push(MIR::ExtractElement(vector.id, index as _, id));

        return Value {
            id,
            ty: T::default(),
            _mark: PhantomData,
        };
    }
    pub fn insert_element<T: ScalarMarkerType, const N: usize>(
        &mut self,
        vector: Value<'ctx, 'func, SIMD<T, N>>,
        value: Value<'ctx, 'func, T>,
        index: u8,
    ) -> Value<'ctx, 'func, SIMD<T, N>>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        if index as usize >= N {
            panic!("index larger then lanes")
        }
        let id = self.block.new_id(vector.ty.to_type());
        self.block
            .inst
            .push(MIR::InsertElement(vector.id, value.id, index as _, id));

        return Value {
            id,
            ty: SIMD::<T, N>::default(),
            _mark: PhantomData,
        };
    }
    /// construct an aggregate type
    pub fn aggregate(
        &mut self,
        ty: AggregateID<'ctx>,
        values: &[Value<'ctx, 'func, Auto<'ctx>>],
    ) -> Value<'ctx, 'func, Aggregate<'ctx>> {
        let agg = self.ctx.get_aggregate(ty);

        if values.len() != agg.fields.len() {
            panic!("invalid arguments")
        }

        for (i, (_key, ty)) in agg.fields.iter().enumerate() {
            if &values[i].ty.inner != ty {
                panic!("mismatch type")
            }
        }
        let id = self.block.new_id(Type::Aggregate(ty));

        self.block
            .inst
            .push(MIR::Aggregate(values.iter().map(|v| v.id).collect(), id));

        return Value {
            id,
            ty: Aggregate(ty),
            _mark: PhantomData,
        };
    }
    pub fn aggregate_to_interface(
        &mut self,
        value: Value<'ctx, 'func, Smart<Aggregate<'ctx>>>,
        iface: InterfaceID<'ctx>,
    ) -> Value<'ctx, 'func, Interface<'ctx>> {
        
        let agg = self.ctx.get_aggregate(value.ty.pointee.0);
        let interface = self.ctx.get_interface(iface);

        if agg.fields.len() < interface.fields.len(){
            panic!("aggregate type does not match interface")
        }

        for (ident, ty) in &interface.fields{
            if agg.fields.iter().find(|(k, t)|k == ident && t == ty).is_none(){
                panic!("aggregate type does not match interface")
            }
        }
        let id = self.block.new_id(Type::Interface(iface));
        self.block.inst.push(MIR::AggregateToInterface(value.id, iface, id));

        return Value { 
            id: id, 
            ty: Interface(iface), 
            _mark: PhantomData
        }
    }
    pub fn interface_to_interface(
        &mut self,
        value: Value<'ctx, 'func, Interface<'ctx>>,
        iface: InterfaceID<'ctx>,
    ) -> Value<'ctx, 'func, Interface<'ctx>> {
        // same interface, no need to map.
        if value.ty.0 == iface {
            return value;
        }

        let iface1 = self.ctx.get_interface(value.ty.0);
        let iface2 = self.ctx.get_interface(iface);

        for (key, ty) in &iface2.fields {
            if !iface1
                .fields
                .iter()
                .find(|(id, t)| id == key && t == ty)
                .is_some()
            {
                panic!("interface mismatch")
            }
        }

        let id = self.block.new_id(Type::Interface(iface));

        // insert instruction
        self.block
            .inst
            .push(MIR::InterfaceToInterface(value.id, iface, id));

        return Value {
            id,
            ty: Interface(iface),
            _mark: PhantomData,
        };
    }

    /// extract a field value from any fielded types.
    /// Accepts aggregate, interface, pointer to aggregate or pointer to interface.
    pub fn extract_value<T: FieldedMarkerType<'ctx>>(
        &mut self,
        target: Value<'ctx, 'func, T>,
        field: Ident,
    ) -> Value<'ctx, 'func, Auto<'ctx>> {
        let ty = match target.ty.to_type() {
            Type::Aggregate(id) => {
                if let Some((_, ty)) = self
                    .ctx
                    .get_aggregate(id)
                    .fields
                    .iter()
                    .find(|(key, _)| key == &field)
                {
                    ty.clone()
                } else {
                    panic!("aggregate has no field")
                }
            }
            Type::Interface(id) => {
                if let Some((_, ty)) = self
                    .ctx
                    .get_interface(id)
                    .fields
                    .iter()
                    .find(|(key, _)| key == &field)
                {
                    ty.clone()
                } else {
                    panic!("interface has no field")
                }
            }
            Type::SmartPointer(p)
            | Type::Pointer(p) => match p.as_ref() {
                Type::Aggregate(id) => {
                    if let Some((_, ty)) = self
                        .ctx
                        .get_aggregate(*id)
                        .fields
                        .iter()
                        .find(|(key, _)| key == &field)
                    {
                        ty.clone()
                    } else {
                        panic!("aggregate has no field")
                    }
                }
                Type::Interface(id) => {
                    if let Some((_, ty)) = self
                        .ctx
                        .get_interface(*id)
                        .fields
                        .iter()
                        .find(|(key, _)| key == &field)
                    {
                        ty.clone()
                    } else {
                        panic!("interface has no field")
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        // the lifetime of type is phanom and can be safely transmuted
        let id = self
            .block
            .new_id(unsafe { core::mem::transmute(ty.clone()) });

        // insert instruction
        self.block.inst.push(MIR::ExtractValue(target.id, field, id));

        return Value {
            id,
            ty: Auto {
                inner: unsafe { core::mem::transmute(ty) },
            },
            _mark: PhantomData,
        };
    }

    /// inserts a value to a field
    /// Accepts aggregate, interface, pointer to aggregate or pointer to interface.
    /// 
    /// if garbage collection is enabled, this will be lowered to a call to write barrier
    pub fn insert_value<T: FieldedMarkerType<'ctx>, V: MarkerType<'ctx>>(
        &mut self,
        target: Value<'ctx, 'func, T>,
        field: Ident,
        value: Value<'ctx, 'func, V>,
    ){
        let ty = match target.ty.to_type() {
            Type::Aggregate(id) => {
                if let Some((_, ty)) = self
                    .ctx
                    .get_aggregate(id)
                    .fields
                    .iter()
                    .find(|(key, _)| key == &field)
                {
                    ty.clone()
                } else {
                    panic!("aggregate has no field")
                }
            }
            Type::Interface(id) => {
                if let Some((_, ty)) = self
                    .ctx
                    .get_interface(id)
                    .fields
                    .iter()
                    .find(|(key, _)| key == &field)
                {
                    ty.clone()
                } else {
                    panic!("interface has no field")
                }
            }
            Type::SmartPointer(p)
            | Type::Pointer(p) => match p.as_ref() {
                Type::Aggregate(id) => {
                    if let Some((_, ty)) = self
                        .ctx
                        .get_aggregate(*id)
                        .fields
                        .iter()
                        .find(|(key, _)| key == &field)
                    {
                        ty.clone()
                    } else {
                        panic!("aggregate has no field")
                    }
                }
                Type::Interface(id) => {
                    if let Some((_, ty)) = self
                        .ctx
                        .get_interface(*id)
                        .fields
                        .iter()
                        .find(|(key, _)| key == &field)
                    {
                        ty.clone()
                    } else {
                        panic!("interface has no field")
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        if value.ty.to_type() != ty{
            panic!("type not match")
        }

        self.block.inst.push(MIR::InsertValue(target.id, field, value.id));

        return;
    }

    pub fn create_stack_slot<T: MarkerType<'ctx>>(&mut self, value: Value<'ctx, 'func, T>) -> StackSlot<'ctx, 'func, T> {
        let id = self.func.stackslots.len();
        self.func.stackslots.push(value.ty.to_type());

        self.block.inst.push(MIR::CreateStackSlot(StackSlotID(id), value.id));

        StackSlot {
            id: StackSlotID(id),
            ty: value.ty,
            _mark: PhantomData,
        }
    }

    pub fn stack_load<T: MarkerType<'ctx>>(
        &mut self,
        slot: StackSlot<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(slot.ty.to_type());
        self.block.inst.push(MIR::StackLoad(slot.id, 0, id));

        return Value {
            id,
            ty: slot.ty.clone(),
            _mark: PhantomData,
        };
    }
    pub fn stack_store<T: MarkerType<'ctx>>(
        &mut self,
        slot: StackSlot<'ctx, 'func, T>,
        value: Value<'ctx, 'func, T>,
    ) {
        self.block.inst.push(MIR::StackStore(slot.id, 0, value.id));
    }

    pub fn stack_pointer<T: MarkerType<'ctx>>(
        &mut self,
        slot: StackSlot<'ctx, 'func, T>,
    ) -> Value<'ctx, 'func, Pointer<T>> {
        let id = self
            .block
            .new_id(Type::Pointer(Box::new(slot.ty.to_type())));
        self.block.inst.push(MIR::StackPtr(slot.id, id));

        return Value {
            id,
            ty: Pointer {
                pointee: slot.ty.clone(),
            },
            _mark: PhantomData,
        };
    }

    pub fn load<T: MarkerType<'ctx>>(
        &mut self,
        ptr: Value<'ctx, 'func, Pointer<T>>,
    ) -> Value<'ctx, 'func, T> {
        let id = self.block.new_id(ptr.ty.pointee.to_type());
        self.block.inst.push(MIR::Load(ptr.id, id));

        return Value {
            id,
            ty: ptr.ty.pointee,
            _mark: PhantomData,
        };
    }
    pub fn store<T: MarkerType<'ctx>>(
        &mut self,
        ptr: Value<'ctx, 'func, Pointer<T>>,
        value: Value<'ctx, 'func, T>,
    ) {
        self.block.inst.push(MIR::Store(ptr.id, value.id));
    }

    pub fn fence(&mut self, ordering: Ordering) {
        self.block.inst.push(MIR::AtomicFence(ordering));
    }
    /// returns the loaded value and success
    pub fn compare_exchange<T: IntMarkerType>(
        &mut self,
        ptr: Value<'ctx, 'func, Pointer<T>>,
        cmp: Value<'ctx, 'func, T>,
        new: Value<'ctx, 'func, T>,
        success_order: Ordering,
        failure_order: Ordering,
    ) -> (Value<'ctx, 'func, T>, Value<'ctx, 'func, I8>) {
        let loaded_id = self.block.new_id(T::default().to_type());
        let success_id = self.block.new_id(Type::I8);

        self.block.inst.push(MIR::AtomicCompareExchange(Box::new((
            ptr.id,
            cmp.id,
            new.id,
            success_order,
            failure_order,
            loaded_id,
            success_id,
        ))));

        return (
            Value {
                id: loaded_id,
                ty: T::default(),
                _mark: PhantomData,
            },
            Value {
                id: success_id,
                ty: I8,
                _mark: PhantomData,
            },
        );
    }

    pub fn jump(&mut self, block: Block<'func>) {
        self.block.inst.push(MIR::Jump(block.id))
    }
    pub fn brz<I: IntMarkerType>(
        &mut self,
        test: Value<'ctx, 'func, I>,
        then: Block<'ctx>,
        else_: Block<'ctx>,
    ) {
        self.block.inst.push(MIR::Brz(test.id, then.id, else_.id));
    }
    pub fn brnz<I: IntMarkerType>(
        &mut self,
        test: Value<'ctx, 'func, I>,
        then: Block<'ctx>,
        else_: Block<'ctx>,
    ) {
        self.block.inst.push(MIR::Brnz(test.id, then.id, else_.id));
    }
    pub fn return_<T: MarkerType<'ctx>>(&mut self, value: Option<Value<'ctx, 'func, T>>) {
        self.block.inst.push(MIR::Return(value.map(|v| v.id)));
    }

    pub fn call_indirect<Arg: types::FunctionArgs<'ctx>, R: MarkerType<'ctx>>(
        &mut self,
        func: Value<'ctx, 'func, types::Function<'ctx, Arg, R>>,
        args: &Arg::ArgValues<'func>,
    ) -> Value<'ctx, 'func, R> {
        
        if args.len() != func.ty.args.len(){
            panic!("arguments not match")
        };


        let id = self.block.new_id(func.ty.return_.to_type());

        return Value { 
            id, 
            ty: func.ty.return_, 
            _mark: PhantomData
        }
    }

    /// this function calls a function directly.
    /// the return value has auto type, user must cast the value before use.
    pub fn call_direct(
        &mut self,
        id: FunctionID<'ctx>,
        args: &[Value<'ctx, 'func, Auto<'ctx>>],
    ) -> Value<'ctx, 'func, Auto<'ctx>> {
        let ty: &FunctionType<'ctx> =
            unsafe { core::mem::transmute(self.ctx.get_function_type(id)) };

        if ty.params.len() != args.len(){
            panic!("number of arguments not match")
        }
        for (i, t) in ty.params.iter().enumerate(){
            if &args[i].ty.inner != t{
                panic!("argument type not match")
            }
        }

        let return_id = self.block.new_id(ty.return_.clone());

        self.block.inst.push(MIR::Call {
            id: FunctionID {
                id: id.id,
                _mark: PhantomData,
            },
            args: args.iter().map(|v|v.id).collect(),
            return_: return_id,
        });

        return Value {
            id: return_id,
            ty: Auto {
                inner: ty.return_.clone(),
            },
            _mark: PhantomData,
        };
    }
}
