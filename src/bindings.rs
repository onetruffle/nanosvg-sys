/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NSVGpaintType {
    NSVG_PAINT_NONE = 0,
    NSVG_PAINT_COLOR = 1,
    NSVG_PAINT_LINEAR_GRADIENT = 2,
    NSVG_PAINT_RADIAL_GRADIENT = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NSVGspreadType {
    NSVG_SPREAD_PAD = 0,
    NSVG_SPREAD_REFLECT = 1,
    NSVG_SPREAD_REPEAT = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NSVGlineJoin {
    NSVG_JOIN_MITER = 0,
    NSVG_JOIN_ROUND = 1,
    NSVG_JOIN_BEVEL = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NSVGlineCap {
    NSVG_CAP_BUTT = 0,
    NSVG_CAP_ROUND = 1,
    NSVG_CAP_SQUARE = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NSVGfillRule {
    NSVG_FILLRULE_NONZERO = 0,
    NSVG_FILLRULE_EVENODD = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NSVGflags { NSVG_FLAGS_VISIBLE = 1, }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct NSVGgradientStop {
    pub color: ::std::os::raw::c_uint,
    pub offset: f32,
}
#[test]
fn bindgen_test_layout_NSVGgradientStop() {
    assert_eq!(::std::mem::size_of::<NSVGgradientStop>() , 8usize , concat ! (
               "Size of: " , stringify ! ( NSVGgradientStop ) ));
    assert_eq! (::std::mem::align_of::<NSVGgradientStop>() , 4usize , concat !
                ( "Alignment of " , stringify ! ( NSVGgradientStop ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradientStop ) ) . color as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradientStop ) ,
                "::" , stringify ! ( color ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradientStop ) ) . offset as * const
                _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradientStop ) ,
                "::" , stringify ! ( offset ) ));
}
impl Clone for NSVGgradientStop {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct NSVGgradient {
    pub xform: [f32; 6usize],
    pub spread: ::std::os::raw::c_char,
    pub fx: f32,
    pub fy: f32,
    pub nstops: ::std::os::raw::c_int,
    pub stops: [NSVGgradientStop; 1usize],
}
#[test]
fn bindgen_test_layout_NSVGgradient() {
    assert_eq!(::std::mem::size_of::<NSVGgradient>() , 48usize , concat ! (
               "Size of: " , stringify ! ( NSVGgradient ) ));
    assert_eq! (::std::mem::align_of::<NSVGgradient>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( NSVGgradient ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradient ) ) . xform as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradient ) , "::" ,
                stringify ! ( xform ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradient ) ) . spread as * const _ as
                usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradient ) , "::" ,
                stringify ! ( spread ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradient ) ) . fx as * const _ as
                usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradient ) , "::" ,
                stringify ! ( fx ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradient ) ) . fy as * const _ as
                usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradient ) , "::" ,
                stringify ! ( fy ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradient ) ) . nstops as * const _ as
                usize } , 36usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradient ) , "::" ,
                stringify ! ( nstops ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGgradient ) ) . stops as * const _ as
                usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGgradient ) , "::" ,
                stringify ! ( stops ) ));
}
impl Clone for NSVGgradient {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct NSVGpaint {
    pub type_: ::std::os::raw::c_char,
    pub __bindgen_anon_1: NSVGpaint__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct NSVGpaint__bindgen_ty_1 {
    pub color: __BindgenUnionField<::std::os::raw::c_uint>,
    pub gradient: __BindgenUnionField<*mut NSVGgradient>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_NSVGpaint__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<NSVGpaint__bindgen_ty_1>() , 8usize ,
               concat ! (
               "Size of: " , stringify ! ( NSVGpaint__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<NSVGpaint__bindgen_ty_1>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( NSVGpaint__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpaint__bindgen_ty_1 ) ) . color as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpaint__bindgen_ty_1
                ) , "::" , stringify ! ( color ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpaint__bindgen_ty_1 ) ) . gradient as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpaint__bindgen_ty_1
                ) , "::" , stringify ! ( gradient ) ));
}
impl Clone for NSVGpaint__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_NSVGpaint() {
    assert_eq!(::std::mem::size_of::<NSVGpaint>() , 16usize , concat ! (
               "Size of: " , stringify ! ( NSVGpaint ) ));
    assert_eq! (::std::mem::align_of::<NSVGpaint>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( NSVGpaint ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpaint ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpaint ) , "::" ,
                stringify ! ( type_ ) ));
}
impl Clone for NSVGpaint {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct NSVGpath {
    pub pts: *mut f32,
    pub npts: ::std::os::raw::c_int,
    pub closed: ::std::os::raw::c_char,
    pub bounds: [f32; 4usize],
    pub next: *mut NSVGpath,
}
#[test]
fn bindgen_test_layout_NSVGpath() {
    assert_eq!(::std::mem::size_of::<NSVGpath>() , 40usize , concat ! (
               "Size of: " , stringify ! ( NSVGpath ) ));
    assert_eq! (::std::mem::align_of::<NSVGpath>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( NSVGpath ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpath ) ) . pts as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpath ) , "::" ,
                stringify ! ( pts ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpath ) ) . npts as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpath ) , "::" ,
                stringify ! ( npts ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpath ) ) . closed as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpath ) , "::" ,
                stringify ! ( closed ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpath ) ) . bounds as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpath ) , "::" ,
                stringify ! ( bounds ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGpath ) ) . next as * const _ as usize
                } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGpath ) , "::" ,
                stringify ! ( next ) ));
}
impl Clone for NSVGpath {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
pub struct NSVGshape {
    pub id: [::std::os::raw::c_char; 64usize],
    pub fill: NSVGpaint,
    pub stroke: NSVGpaint,
    pub opacity: f32,
    pub strokeWidth: f32,
    pub strokeDashOffset: f32,
    pub strokeDashArray: [f32; 8usize],
    pub strokeDashCount: ::std::os::raw::c_char,
    pub strokeLineJoin: ::std::os::raw::c_char,
    pub strokeLineCap: ::std::os::raw::c_char,
    pub miterLimit: f32,
    pub fillRule: ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_uchar,
    pub bounds: [f32; 4usize],
    pub paths: *mut NSVGpath,
    pub next: *mut NSVGshape,
}
#[test]
fn bindgen_test_layout_NSVGshape() {
    assert_eq!(::std::mem::size_of::<NSVGshape>() , 184usize , concat ! (
               "Size of: " , stringify ! ( NSVGshape ) ));
    assert_eq! (::std::mem::align_of::<NSVGshape>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( NSVGshape ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . id as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( id ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . fill as * const _ as
                usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( fill ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . stroke as * const _ as
                usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( stroke ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . opacity as * const _ as
                usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( opacity ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . strokeWidth as * const _
                as usize } , 100usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( strokeWidth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . strokeDashOffset as *
                const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( strokeDashOffset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . strokeDashArray as *
                const _ as usize } , 108usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( strokeDashArray ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . strokeDashCount as *
                const _ as usize } , 140usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( strokeDashCount ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . strokeLineJoin as * const
                _ as usize } , 141usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( strokeLineJoin ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . strokeLineCap as * const
                _ as usize } , 142usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( strokeLineCap ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . miterLimit as * const _
                as usize } , 144usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( miterLimit ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . fillRule as * const _ as
                usize } , 148usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( fillRule ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . flags as * const _ as
                usize } , 149usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( flags ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . bounds as * const _ as
                usize } , 152usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( bounds ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . paths as * const _ as
                usize } , 168usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( paths ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGshape ) ) . next as * const _ as
                usize } , 176usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGshape ) , "::" ,
                stringify ! ( next ) ));
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct NSVGimage {
    pub width: f32,
    pub height: f32,
    pub shapes: *mut NSVGshape,
}
#[test]
fn bindgen_test_layout_NSVGimage() {
    assert_eq!(::std::mem::size_of::<NSVGimage>() , 16usize , concat ! (
               "Size of: " , stringify ! ( NSVGimage ) ));
    assert_eq! (::std::mem::align_of::<NSVGimage>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( NSVGimage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGimage ) ) . width as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGimage ) , "::" ,
                stringify ! ( width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGimage ) ) . height as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGimage ) , "::" ,
                stringify ! ( height ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NSVGimage ) ) . shapes as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( NSVGimage ) , "::" ,
                stringify ! ( shapes ) ));
}
impl Clone for NSVGimage {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn nsvgParseFromFile(filename: *const ::std::os::raw::c_char,
                             units: *const ::std::os::raw::c_char, dpi: f32)
     -> *mut NSVGimage;
}
extern "C" {
    pub fn nsvgParse(input: *mut ::std::os::raw::c_char,
                     units: *const ::std::os::raw::c_char, dpi: f32)
     -> *mut NSVGimage;
}
extern "C" {
    pub fn nsvgDelete(image: *mut NSVGimage);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NSVGrasterizer {
    _unused: [u8; 0],
}
extern "C" {
    pub fn nsvgCreateRasterizer() -> *mut NSVGrasterizer;
}
extern "C" {
    pub fn nsvgRasterize(r: *mut NSVGrasterizer, image: *mut NSVGimage,
                         tx: f32, ty: f32, scale: f32,
                         dst: *mut ::std::os::raw::c_uchar,
                         w: ::std::os::raw::c_int, h: ::std::os::raw::c_int,
                         stride: ::std::os::raw::c_int);
}
extern "C" {
    pub fn nsvgDeleteRasterizer(arg1: *mut NSVGrasterizer);
}