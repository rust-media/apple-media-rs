use core_foundation::{
    base::{Boolean, CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
};

use crate::{
    buffer::TCVBuffer,
    image_buffer::{CVImageBufferRef, TCVImageBuffer},
    GLenum, GLuint,
};

pub type CVOpenGLTextureRef = CVImageBufferRef;

extern "C" {
    pub fn CVOpenGLTextureGetTypeID() -> CFTypeID;
    pub fn CVOpenGLTextureRetain(texture: CVOpenGLTextureRef) -> CVOpenGLTextureRef;
    pub fn CVOpenGLTextureRelease(texture: CVOpenGLTextureRef);
    pub fn CVOpenGLTextureGetTarget(image: CVOpenGLTextureRef) -> GLenum;
    pub fn CVOpenGLTextureGetName(image: CVOpenGLTextureRef) -> GLuint;
    pub fn CVOpenGLTextureIsFlipped(image: CVOpenGLTextureRef) -> Boolean;
    pub fn CVOpenGLTextureGetCleanTexCoords(
        image: CVOpenGLTextureRef,
        lowerLeft: *mut f32,
        lowerRight: *mut f32,
        upperRight: *mut f32,
        upperLeft: *mut f32,
    );
}

impl TCVBuffer for CVOpenGLTexture {}
impl TCVImageBuffer for CVOpenGLTexture {}

pub struct CVOpenGLTexture(CVOpenGLTextureRef);

impl Drop for CVOpenGLTexture {
    fn drop(&mut self) {
        unsafe { CVOpenGLTextureRelease(self.0) }
    }
}

impl_TCFType!(CVOpenGLTexture, CVOpenGLTextureRef, CVOpenGLTextureGetTypeID);
impl_CFTypeDescription!(CVOpenGLTexture);

impl CVOpenGLTexture {
    #[inline]
    pub fn get_target(&self) -> GLenum {
        unsafe { CVOpenGLTextureGetTarget(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_name(&self) -> GLuint {
        unsafe { CVOpenGLTextureGetName(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn is_flipped(&self) -> bool {
        unsafe { CVOpenGLTextureIsFlipped(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn get_clean_tex_coords(&self) -> (f32, f32, f32, f32) {
        let mut lower_left = 0.0;
        let mut lower_right = 0.0;
        let mut upper_right = 0.0;
        let mut upper_left = 0.0;
        unsafe {
            CVOpenGLTextureGetCleanTexCoords(self.as_concrete_TypeRef(), &mut lower_left, &mut lower_right, &mut upper_right, &mut upper_left);
        }
        (lower_left, lower_right, upper_right, upper_left)
    }
}
