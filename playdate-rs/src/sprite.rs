use core::cell::RefCell;

use alloc::vec::Vec;
use playdate_rs_sys::LCDPattern;

use crate::{
    graphics::{so2d_to_lcdrect, Bitmap},
    math::{Point2D, Rect, SideOffsets2D, Vec2D},
    util::Ref,
    PLAYDATE,
};

pub use sys::SpriteCollisionResponseType;

fn rect_to_pdrect(rect: Rect<f32>) -> sys::PDRect {
    sys::PDRect {
        x: rect.origin.x,
        y: rect.origin.y,
        width: rect.size.width,
        height: rect.size.height,
    }
}

fn pdrect_to_rect(rect: sys::PDRect) -> Rect<f32> {
    Rect {
        origin: (rect.x, rect.y).into(),
        size: (rect.width, rect.height).into(),
    }
}

fn collision_point_to_point(point: sys::CollisionPoint) -> Point2D<f32> {
    Point2D::new(point.x, point.y)
}

fn collision_vec_to_vec(vec: sys::CollisionVector) -> Vec2D<i32> {
    Vec2D::new(vec.x, vec.y)
}

pub struct PlaydateSprite {
    handle: *const sys::playdate_sprite,
}

impl PlaydateSprite {
    pub(crate) fn new(handle: *const sys::playdate_sprite) -> Self {
        Self { handle }
    }

    /// When flag is set to 1, the given sprite will always redraw.
    pub fn set_always_redraw(&self, flag: bool) {
        unsafe {
            (*self.handle).setAlwaysRedraw.unwrap()(flag as i32);
        }
    }

    /// Marks the given dirtyRect (in screen coordinates) as needing a redraw. Graphics drawing functions now call this automatically, adding their drawn areas to the sprite’s dirty list, so there’s usually no need to call this manually.
    pub fn add_dirty_rect(&self, dirty_rect: SideOffsets2D<i32>) {
        unsafe {
            (*self.handle).addDirtyRect.unwrap()(so2d_to_lcdrect(dirty_rect));
        }
    }

    /// Draws every sprite in the display list.
    pub fn draw_sprites(&self) {
        unsafe {
            (*self.handle).drawSprites.unwrap()();
        }
    }

    /// Updates and draws every sprite in the display list.
    pub fn update_and_draw_sprites(&self) {
        unsafe {
            (*self.handle).updateAndDrawSprites.unwrap()();
        }
    }

    /// Allocates and returns a new LCDSprite.
    pub fn new_sprite(&self) -> Sprite {
        Sprite::from(unsafe { (*self.handle).newSprite.unwrap()() })
    }

    /// Adds the given sprite to the display list, so that it is drawn in the current scene.
    pub fn add_sprite(&self, sprite: impl AsRef<Sprite>) {
        unsafe {
            (*self.handle).addSprite.unwrap()(sprite.as_ref().handle);
        }
    }

    /// Removes the given sprite from the display list.
    pub fn remove_sprite(&self, sprite: impl AsRef<Sprite>) {
        unsafe {
            (*self.handle).removeSprite.unwrap()(sprite.as_ref().handle);
        }
    }

    /// Removes the given count sized array of sprites from the display list.
    pub fn remove_sprites(&self, sprites: &[&Sprite]) {
        let mut sprites: Vec<_> = sprites.iter().map(|s| s.handle).collect();
        unsafe {
            (*self.handle).removeSprites.unwrap()(sprites.as_mut_ptr(), sprites.len() as i32);
        }
    }

    /// Removes all sprites from the display list.
    pub fn remove_all_sprites(&self) {
        unsafe {
            (*self.handle).removeAllSprites.unwrap()();
        }
    }

    /// Returns the total number of sprites in the display list.
    pub fn get_sprite_count(&self) -> usize {
        unsafe { (*self.handle).getSpriteCount.unwrap()() as _ }
    }

    /// Sets the clipping rectangle for all sprites with a Z index within startZ and endZ inclusive.
    pub fn set_clip_rects_in_range(&self, clip_rect: SideOffsets2D<i32>, start_z: i32, end_z: i32) {
        unsafe {
            (*self.handle).setClipRectsInRange.unwrap()(so2d_to_lcdrect(clip_rect), start_z, end_z);
        }
    }

    /// Clears the clipping rectangle for all sprites with a Z index within startZ and endZ inclusive.
    pub fn clear_clip_rects_in_range(&self, start_z: i32, end_z: i32) {
        unsafe {
            (*self.handle).clearClipRectsInRange.unwrap()(start_z, end_z);
        }
    }

    /// Frees and reallocates internal collision data, resetting everything to its default state.
    pub fn reset_collision_world(&self) {
        unsafe {
            (*self.handle).resetCollisionWorld.unwrap()();
        }
    }

    /// Returns the same values as playdate->sprite->moveWithCollisions() but does not actually move the sprite.
    pub fn check_collisions(
        &self,
        sprite: impl AsRef<Sprite>,
        goal_x: f32,
        goal_y: f32,
    ) -> Vec<SpriteCollisionInfo> {
        let mut actual_x = 0.0;
        let mut actual_y = 0.0;
        let mut len = 0;
        let info = unsafe {
            (*self.handle).checkCollisions.unwrap()(
                sprite.as_ref().handle,
                goal_x,
                goal_y,
                &mut actual_x,
                &mut actual_y,
                &mut len,
            )
        };
        let mut result = Vec::new();
        for i in 0..len {
            let info = unsafe { info.offset(i as isize).as_ref().unwrap() };
            result.push(SpriteCollisionInfo::new(info));
        }
        // caller is responsible for freeing memory of array returned by moveWithCollisions()
        PLAYDATE.system.realloc(info as _, 0);
        result
    }

    /// Moves the given sprite towards goalX, goalY taking collisions into account and returns an array of SpriteCollisionInfo. len is set to the size of the array and actualX, actualY are set to the sprite’s position after collisions. If no collisions occurred, this will be the same as goalX, goalY.
    pub fn move_with_collisions(
        &self,
        sprite: impl AsRef<Sprite>,
        goal_x: f32,
        goal_y: f32,
    ) -> Vec<SpriteCollisionInfo> {
        let mut actual_x = 0.0;
        let mut actual_y = 0.0;
        let mut len = 0;
        let info = unsafe {
            (*self.handle).moveWithCollisions.unwrap()(
                sprite.as_ref().handle,
                goal_x,
                goal_y,
                &mut actual_x,
                &mut actual_y,
                &mut len,
            )
        };
        let mut result = Vec::new();
        for i in 0..len {
            let info = unsafe { info.offset(i as isize).as_ref().unwrap() };
            result.push(SpriteCollisionInfo::new(info));
        }
        // caller is responsible for freeing memory of array returned by moveWithCollisions()
        PLAYDATE.system.realloc(info as _, 0);
        result
    }

    pub fn query_sprites_at_point(&self, x: f32, y: f32) -> Vec<Ref<Sprite>> {
        let mut len = 0;
        let sprites = unsafe { (*self.handle).querySpritesAtPoint.unwrap()(x, y, &mut len) };
        let mut result = Vec::new();
        for i in 0..len {
            let sprite = unsafe { sprites.offset(i as isize).as_ref().unwrap() };
            result.push(Sprite::from_ref(*sprite));
        }
        PLAYDATE.system.realloc(sprites as _, 0);
        result
    }

    pub fn query_sprites_in_rect(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Vec<Ref<Sprite>> {
        let mut len = 0;
        let sprites =
            unsafe { (*self.handle).querySpritesInRect.unwrap()(x, y, width, height, &mut len) };
        let mut result = Vec::new();
        for i in 0..len {
            let sprite = unsafe { sprites.offset(i as isize).as_ref().unwrap() };
            result.push(Sprite::from_ref(*sprite));
        }
        PLAYDATE.system.realloc(sprites as _, 0);
        result
    }

    pub fn query_sprites_along_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Vec<Ref<Sprite>> {
        let mut len = 0;
        let sprites =
            unsafe { (*self.handle).querySpritesAlongLine.unwrap()(x1, y1, x2, y2, &mut len) };
        let mut result = Vec::new();
        for i in 0..len {
            let sprite = unsafe { sprites.offset(i as isize).as_ref().unwrap() };
            result.push(Sprite::from_ref(*sprite));
        }
        PLAYDATE.system.realloc(sprites as _, 0);
        result
    }

    pub fn query_sprite_info_along_line(
        &self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> Vec<SpriteQueryInfo> {
        let mut len = 0;
        let info =
            unsafe { (*self.handle).querySpriteInfoAlongLine.unwrap()(x1, y1, x2, y2, &mut len) };
        let mut result = Vec::new();
        for i in 0..len {
            let info = unsafe { info.offset(i as isize).as_ref().unwrap() };
            result.push(SpriteQueryInfo::new(info));
        }
        PLAYDATE.system.realloc(info as _, 0);
        result
    }

    /// Returns an array of sprites that have collide rects that are currently overlapping the given sprite’s collide rect.
    pub fn overlapping_sprites(&self, sprite: impl AsRef<Sprite>) -> Vec<Ref<Sprite>> {
        let mut len = 0;
        let sprites =
            unsafe { (*self.handle).overlappingSprites.unwrap()(sprite.as_ref().handle, &mut len) };
        let mut result = Vec::new();
        for i in 0..len {
            let sprite = unsafe { sprites.offset(i as isize).as_ref().unwrap() };
            result.push(Sprite::from_ref(*sprite));
        }
        PLAYDATE.system.realloc(sprites as _, 0);
        result
    }

    /// Returns an array of all sprites that have collide rects that are currently overlapping. Each consecutive pair of sprites is overlapping (eg. 0 & 1 overlap, 2 & 3 overlap, etc).
    pub fn all_overlapping_sprites(&self) -> Vec<Ref<Sprite>> {
        let mut len = 0;
        let sprites = unsafe { (*self.handle).allOverlappingSprites.unwrap()(&mut len) };
        let mut result = Vec::new();
        for i in 0..len {
            let sprite = unsafe { sprites.offset(i as isize).as_ref().unwrap() };
            result.push(Sprite::from_ref(*sprite));
        }
        PLAYDATE.system.realloc(sprites as _, 0);
        result
    }
}

#[derive(Debug)]
pub struct Sprite {
    handle: *mut sys::LCDSprite,
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Sprite {}

unsafe impl Sync for Sprite {}
unsafe impl Send for Sprite {}

impl Default for Sprite {
    fn default() -> Self {
        Self::new()
    }
}

type DataCell<F> = RefCell<Option<F>>;
type UpdateFn = Box<dyn Fn(&Sprite)>;
type DrawFn = Box<dyn Fn(&Sprite, Rect<f32>, Rect<f32>)>;
type CollisionResponseFn = Box<dyn Fn(&Sprite, &Sprite) -> SpriteCollisionResponseType>;

#[derive(Default)]
struct SpriteData {
    update_fn: DataCell<UpdateFn>,
    draw_fn: DataCell<DrawFn>,
    collision_response_fn: DataCell<CollisionResponseFn>,
}

impl Sprite {
    pub(crate) fn from(handle: *mut sys::LCDSprite) -> Self {
        Self { handle }
    }

    pub(crate) fn from_ref<'a>(handle: *mut sys::LCDSprite) -> Ref<'a, Self> {
        Ref::new(Self { handle })
    }

    /// Allocates and returns a new Sprite.
    pub fn new() -> Self {
        PLAYDATE.sprite.new_sprite()
    }

    /// Sets x and y to the current position of sprite.
    pub fn get_position(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            (*PLAYDATE.sprite.handle).getPosition.unwrap()(self.handle, &mut x, &mut y);
        }
        (x, y)
    }

    /// Sets the bounds of the given sprite with bounds.
    pub fn set_bounds(&self, bounds: Rect<f32>) {
        unsafe { (*PLAYDATE.sprite.handle).setBounds.unwrap()(self.handle, rect_to_pdrect(bounds)) }
    }

    /// Returns the bounds of the given sprite as an PDRect;
    pub fn get_bounds(&self) -> Rect<f32> {
        unsafe { pdrect_to_rect((*PLAYDATE.sprite.handle).getBounds.unwrap()(self.handle)) }
    }

    /// Moves the given sprite to x, y and resets its bounds based on the bitmap dimensions and center.
    pub fn move_to(&self, x: f32, y: f32) {
        unsafe { (*PLAYDATE.sprite.handle).moveTo.unwrap()(self.handle, x, y) }
    }

    /// Moves the given sprite to by offsetting its current position by dx, dy.
    pub fn move_by(&self, dx: f32, dy: f32) {
        unsafe { (*PLAYDATE.sprite.handle).moveBy.unwrap()(self.handle, dx, dy) }
    }

    /// Sets the given sprite's image to the given bitmap.
    pub fn set_image(&self, image: Bitmap, flip: sys::LCDBitmapFlip) {
        // drop old image
        if let Some(old_image) = self.get_image() {
            let _boxed = Bitmap::from(old_image.handle);
        }
        // set new image. pass the ownership to the system
        unsafe {
            (*PLAYDATE.sprite.handle).setImage.unwrap()(self.handle, image.as_ref().handle, flip)
        };
        // forget image so it doesn't get dropped
        core::mem::forget(image);
    }

    /// Returns the LCDBitmap currently assigned to the given sprite.
    pub fn get_image(&self) -> Option<Ref<Bitmap>> {
        let ptr = unsafe { (*PLAYDATE.sprite.handle).getImage.unwrap()(self.handle) };
        if ptr.is_null() {
            None
        } else {
            Some(Bitmap::from_ref(ptr))
        }
    }

    /// Sets the size. The size is used to set the sprite’s bounds when calling moveTo().
    pub fn set_size(&self, width: f32, height: f32) {
        unsafe { (*PLAYDATE.sprite.handle).setSize.unwrap()(self.handle, width, height) }
    }

    /// Sets the Z order of the given sprite. Higher Z sprites are drawn on top of those with lower Z order.
    pub fn set_z_index(&self, z_index: i16) {
        unsafe { (*PLAYDATE.sprite.handle).setZIndex.unwrap()(self.handle, z_index) }
    }

    /// Returns the Z index of the given sprite.
    pub fn get_z_index(&self) -> i16 {
        unsafe { (*PLAYDATE.sprite.handle).getZIndex.unwrap()(self.handle) }
    }

    /// Sets the mode for drawing the sprite’s bitmap.
    pub fn set_draw_mode(&self, mode: sys::LCDBitmapDrawMode) {
        unsafe { (*PLAYDATE.sprite.handle).setDrawMode.unwrap()(self.handle, mode) }
    }

    /// Flips the bitmap.
    pub fn set_image_flip(&self, flip: sys::LCDBitmapFlip) {
        unsafe { (*PLAYDATE.sprite.handle).setImageFlip.unwrap()(self.handle, flip) }
    }

    /// Returns the flip setting of the sprite’s bitmap.
    pub fn get_image_flip(&self) -> sys::LCDBitmapFlip {
        unsafe { (*PLAYDATE.sprite.handle).getImageFlip.unwrap()(self.handle) }
    }

    /// Specifies a stencil image to be set on the frame buffer before the sprite is drawn.
    pub fn set_stencil(&self, stencil: impl AsRef<Bitmap>) {
        unsafe {
            (*PLAYDATE.sprite.handle).setStencil.unwrap()(self.handle, stencil.as_ref().handle)
        };
    }

    /// Sets the clipping rectangle for sprite drawing.
    pub fn set_clip_rect(&self, clip_rect: SideOffsets2D<i32>) {
        unsafe {
            (*PLAYDATE.sprite.handle).setClipRect.unwrap()(self.handle, so2d_to_lcdrect(clip_rect))
        };
    }

    /// Clears the sprite’s clipping rectangle.
    pub fn clear_clip_rect(&self) {
        unsafe { (*PLAYDATE.sprite.handle).clearClipRect.unwrap()(self.handle) };
    }

    /// Set the updatesEnabled flag of the given sprite (determines whether the sprite has its update function called).
    pub fn set_updates_enabled(&self, flag: bool) {
        unsafe { (*PLAYDATE.sprite.handle).setUpdatesEnabled.unwrap()(self.handle, flag as _) };
    }

    /// Get the updatesEnabled flag of the given sprite.
    pub fn updates_enabled(&self) -> bool {
        unsafe { (*PLAYDATE.sprite.handle).updatesEnabled.unwrap()(self.handle) == 1 }
    }

    /// Set the collisionsEnabled flag of the given sprite (along with the collideRect, this determines whether the sprite participates in collisions). Set to true by default.
    pub fn set_collisions_enabled(&self, flag: bool) {
        unsafe { (*PLAYDATE.sprite.handle).setCollisionsEnabled.unwrap()(self.handle, flag as _) };
    }

    /// Get the collisionsEnabled flag of the given sprite.
    pub fn collisions_enabled(&self) -> bool {
        unsafe { (*PLAYDATE.sprite.handle).collisionsEnabled.unwrap()(self.handle) == 1 }
    }

    /// Set the visible flag of the given sprite (determines whether the sprite has its draw function called).
    pub fn set_visible(&self, flag: bool) {
        unsafe { (*PLAYDATE.sprite.handle).setVisible.unwrap()(self.handle, flag as _) };
    }

    /// Get the visible flag of the given sprite.
    pub fn is_visible(&self) -> bool {
        unsafe { (*PLAYDATE.sprite.handle).isVisible.unwrap()(self.handle) == 1 }
    }

    /// Marking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite, since it will be overdrawn anyway. If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag.
    pub fn set_opaque(&self, flag: bool) {
        unsafe { (*PLAYDATE.sprite.handle).setOpaque.unwrap()(self.handle, flag as _) };
    }

    /// Forces the given sprite to redraw.
    pub fn mark_dirty(&self) {
        unsafe { (*PLAYDATE.sprite.handle).markDirty.unwrap()(self.handle) };
    }

    /// Sets the tag of the given sprite. This can be useful for identifying sprites or types of sprites when using the collision API.
    pub fn set_tag(&self, tag: u8) {
        unsafe { (*PLAYDATE.sprite.handle).setTag.unwrap()(self.handle, tag) }
    }

    /// Returns the tag of the given sprite.
    pub fn get_tag(&self) -> u8 {
        unsafe { (*PLAYDATE.sprite.handle).getTag.unwrap()(self.handle) }
    }

    /// When flag is set to 1, the sprite will draw in screen coordinates, ignoring the currently-set drawOffset.
    ///
    /// This only affects drawing, and should not be used on sprites being used for collisions, which will still happen in world-space.
    pub fn set_ignores_draw_offset(&self, flag: i32) {
        unsafe { (*PLAYDATE.sprite.handle).setIgnoresDrawOffset.unwrap()(self.handle, flag) };
    }

    /// Sets the update function for the given sprite.
    pub fn set_update_function(&self, func: impl Fn(&Sprite) + 'static) {
        *self.get_userdata().update_fn.borrow_mut() = Some(Box::new(func));
        extern "C" fn callback(sprite: *mut sys::LCDSprite) {
            let sprite = Sprite::from_ref(sprite);
            let func = sprite.get_userdata().update_fn.borrow();
            let func = &func.as_ref().unwrap();
            func(&sprite)
        }
        unsafe {
            (*PLAYDATE.sprite.handle).setUpdateFunction.unwrap()(self.handle, Some(callback))
        };
    }

    /// Sets the draw function for the given sprite.
    pub fn set_draw_function(&self, func: impl Fn(&Sprite, Rect<f32>, Rect<f32>) + 'static) {
        *self.get_userdata().draw_fn.borrow_mut() = Some(Box::new(func));
        extern "C" fn callback(
            sprite: *mut sys::LCDSprite,
            bounds: sys::PDRect,
            drawrect: sys::PDRect,
        ) {
            let sprite = Sprite::from_ref(sprite);
            let func = sprite.get_userdata().draw_fn.borrow();
            let func = &func.as_ref().unwrap();
            func(&sprite, pdrect_to_rect(bounds), pdrect_to_rect(drawrect))
        }
        unsafe { (*PLAYDATE.sprite.handle).setDrawFunction.unwrap()(self.handle, Some(callback)) };
    }

    /// Marks the area of the given sprite, relative to its bounds, to be checked for collisions with other sprites' collide rects.
    pub fn set_collide_rect(&self, collide_rect: Rect<f32>) {
        unsafe {
            (*PLAYDATE.sprite.handle).setCollideRect.unwrap()(
                self.handle,
                rect_to_pdrect(collide_rect),
            );
        }
    }

    /// Returns the given sprite’s collide rect.
    pub fn get_collide_rect(&self) -> Rect<f32> {
        unsafe {
            pdrect_to_rect((*PLAYDATE.sprite.handle).getCollideRect.unwrap()(
                self.handle,
            ))
        }
    }

    /// Clears the given sprite’s collide rect.
    pub fn clear_collide_rect(&self) {
        unsafe { (*PLAYDATE.sprite.handle).clearCollideRect.unwrap()(self.handle) };
    }

    /// Set a callback that returns a SpriteCollisionResponseType for a collision between sprite and other.
    pub fn set_collision_response_function(
        &self,
        func: impl Fn(&Sprite, &Sprite) -> SpriteCollisionResponseType + 'static,
    ) {
        *self.get_userdata().collision_response_fn.borrow_mut() = Some(Box::new(func));
        extern "C" fn callback(
            sprite: *mut sys::LCDSprite,
            other: *mut sys::LCDSprite,
        ) -> SpriteCollisionResponseType {
            let sprite = Sprite::from_ref(sprite);
            let other = Sprite::from_ref(other);
            let func = sprite.get_userdata().collision_response_fn.borrow();
            let func = &func.as_ref().unwrap();
            func(&sprite, &other)
        }
        unsafe {
            (*PLAYDATE.sprite.handle)
                .setCollisionResponseFunction
                .unwrap()(self.handle, Some(callback));
        }
    }

    /// Sets the sprite’s stencil to the given pattern.
    pub fn set_stencil_pattern(&self, mut pattern: LCDPattern) {
        unsafe {
            (*PLAYDATE.sprite.handle).setStencilPattern.unwrap()(self.handle, pattern.as_mut_ptr())
        };
    }

    /// Clears the sprite’s stencil.
    pub fn clear_stencil(&self) {
        unsafe { (*PLAYDATE.sprite.handle).clearStencil.unwrap()(self.handle) };
    }

    /// Gets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data.
    fn get_userdata(&self) -> &SpriteData {
        let ptr = unsafe { (*PLAYDATE.sprite.handle).getUserdata.unwrap()(self.handle) };
        if ptr.is_null() {
            let ptr = Box::into_raw(Box::<SpriteData>::default());
            unsafe { (*PLAYDATE.sprite.handle).setUserdata.unwrap()(self.handle, ptr as _) };
        }
        let ptr = unsafe { (*PLAYDATE.sprite.handle).getUserdata.unwrap()(self.handle) };
        unsafe { &*(ptr as *mut SpriteData) }
    }

    fn drop_userdata(&self) {
        let ptr = unsafe { (*PLAYDATE.sprite.handle).getUserdata.unwrap()(self.handle) };
        if !ptr.is_null() {
            let _boxed = unsafe { Box::from_raw(ptr as *mut SpriteData) };
        }
    }

    /// Specifies a stencil image to be set on the frame buffer before the sprite is drawn. If tile is set, the stencil will be tiled. Tiled stencils must have width evenly divisible by 32.
    pub fn set_stencil_image(&self, stencil: impl AsRef<Bitmap>, tile: i32) {
        unsafe {
            (*PLAYDATE.sprite.handle).setStencilImage.unwrap()(
                self.handle,
                stencil.as_ref().handle,
                tile,
            )
        };
    }
}

impl AsRef<Self> for Sprite {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        // FIXME: Remove from display list?
        self.drop_userdata();
        unsafe { (*PLAYDATE.sprite.handle).freeSprite.unwrap()(self.handle) };
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        let handle = unsafe { (*PLAYDATE.sprite.handle).copy.unwrap()(self.handle) };
        Self { handle }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteCollisionInfo<'a> {
    pub sprite: Ref<'a, Sprite>,
    pub other: Ref<'a, Sprite>,
    pub response_type: SpriteCollisionResponseType,
    pub overlaps: u8,
    pub ti: f32,
    pub move_: Point2D<f32>,
    pub normal: Vec2D<i32>,
    pub touch: Point2D<f32>,
    pub sprite_rect: Rect<f32>,
    pub other_rect: Rect<f32>,
}

impl<'a> SpriteCollisionInfo<'a> {
    fn new(info: &sys::SpriteCollisionInfo) -> Self {
        Self {
            sprite: Sprite::from_ref(info.sprite),
            other: Sprite::from_ref(info.other),
            response_type: info.responseType,
            overlaps: info.overlaps,
            ti: info.ti,
            move_: collision_point_to_point(info.move_),
            normal: collision_vec_to_vec(info.normal),
            touch: collision_point_to_point(info.touch),
            sprite_rect: pdrect_to_rect(info.spriteRect),
            other_rect: pdrect_to_rect(info.otherRect),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteQueryInfo<'a> {
    pub sprite: Ref<'a, Sprite>,
    pub ti1: f32,
    pub ti2: f32,
    pub entry_point: Point2D<f32>,
    pub exit_point: Point2D<f32>,
}

impl<'a> SpriteQueryInfo<'a> {
    fn new(info: &sys::SpriteQueryInfo) -> Self {
        Self {
            sprite: Sprite::from_ref(info.sprite),
            ti1: info.ti1,
            ti2: info.ti2,
            entry_point: collision_point_to_point(info.entryPoint),
            exit_point: collision_point_to_point(info.exitPoint),
        }
    }
}
