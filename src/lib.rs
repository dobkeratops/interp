use std::ops::{Add,Sub,Mul,Div};
#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {	
		let a=10.0f32;
		let b=20.0f32;
		let f0=0.25f32;
		let f1=0.75f32;
		let f=0.5f32;
		let z1=a.lerp(&b,f0);
		let z2=a.lerp(&b,f1);
		let w1=12.5f32;
		let w2=17.5f32;
        assert_eq!(z1,w1);
        assert_eq!(z2,w2);
        assert_eq!(z1.inv_lerp(&a,&b),f0);
        assert_eq!(z2.inv_lerp(&a,&b),f1);
    }
}

/// linear interpolation trait,
/// blends between self and the first parameter,
// by the second parameter
/// uses reference args for suitability with heavier types
pub trait Lerp<F=f32> :Sized{
	fn lerp(&self,b:&Self,factor:F)->Self;
	fn lerp_points<Y>(&self,a:&Self,b:&Self, y0:&Y,y1:&Y)->Y
		where Y:Lerp<F>, Self:InvLerp<F>
	{
		lerp_points(self,a,b, y0,y1)
	}
}
/// Linear interpolation, reversed order params
/// 'self' is the blend factor; 
/// trailing params are the range
pub trait LerpBetween<T> :Sized+Clone where T:Lerp<Self>{
	fn lerp_between(&self,a:&T,b:&T)->T {
		a.lerp(b,self.clone())
	}
}
/// free-function wrapper for 'lerp'
pub fn lerp<F,T:Lerp<F>>(a:&T,b:&T, f:F)->T{
	a.lerp(b,f)
}
/// Inverse linear interpolation trait,
/// gives the factor by which 'self'
/// is between the range specified by the args
pub trait InvLerp<F=f32>:Sized {
	fn inv_lerp(&self,a:&Self,b:&Self)->F;
}

/// bigger formulation of 'lerp' with blend factor
/// taking known points,
/// https://en.wikipedia.org/wiki/Linear_interpolation#Linear_interpolation_between_two_known_points
/// implementable through lerp/invlerp
/// TODO but also possible for int types ordered
/// differently i.e multiply-add - divide by x diff
fn lerp_points<X,Y,F>(x:&X,x0:&X,x1:&X, y0:&Y,y1:&Y)->Y where
	X:InvLerp<F>, Y:Lerp<F>
{
	y0.lerp(y1, x.inv_lerp(x0,x1) )
}

/// generic implementation which should work for propogation of
/// dimensional intermediate types, fraction/fixed point types, etc
impl<T,Diff,Scaled,Factor:Copy> Lerp<Factor> for T where
	for<'u,'v> &'u T:Sub<&'v T,Output=Diff>,
	for<'u,'v> &'u Diff:Mul<Factor,Output=Scaled>,
//	for<'u,'v> &'u DiffScaled:Add<&'v T,Output=T>,
	for<'u,'v,'w> &'u T:Add<&'v Scaled,Output=T>,   
	//<Diff as Mul<&'w Factor>>::Output
{
	fn lerp(&self,b:&Self,factor:Factor)->Self{
		let diff=b.sub(self);
		let scaled=diff.mul(factor);
		self.add(&scaled)
	}
}	
impl<T,Diff,Factor> InvLerp<Factor> for T where
	for <'u,'v> &'u T:Sub<&'v T,Output=Diff>,
	for <'u,'v> &'u Diff:Div<&'v Diff, Output=Factor>
{
	fn inv_lerp(&self,a:&Self,b:&Self)->Factor{
		let rng=b.sub(a);
		let ofs=self.sub(a);
		ofs.div(&rng)
	}
}

pub fn avr<T:Lerp>(a:&T,b:&T)->T{ a.lerp(b,0.5f32) }

