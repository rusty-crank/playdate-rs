use alloc::vec::Vec;

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

pub struct _Sprite {
    handle: *const sys::playdate_sprite,
}

impl _Sprite {
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

    /// Frees the given sprite.
    pub(crate) fn free_sprite(&self, sprite: *mut sys::LCDSprite) {
        unsafe {
            (*self.handle).freeSprite.unwrap()(sprite);
        }
    }

    /// Allocates and returns a copy of the given sprite.
    pub(crate) fn copy(&self, sprite: *mut sys::LCDSprite) -> *mut sys::LCDSprite {
        unsafe { (*self.handle).copy.unwrap()(sprite) }
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
    pub fn get_sprite_count(&self) -> i32 {
        unsafe { (*self.handle).getSpriteCount.unwrap()() }
    }

    /// Sets the bounds of the given sprite with bounds.
    pub(crate) fn set_bounds(&self, sprite: *mut sys::LCDSprite, bounds: Rect<f32>) {
        unsafe {
            (*self.handle).setBounds.unwrap()(sprite, rect_to_pdrect(bounds));
        }
    }

    /// Returns the bounds of the given sprite as an PDRect;
    pub(crate) fn get_bounds(&self, sprite: *mut sys::LCDSprite) -> Rect<f32> {
        unsafe { pdrect_to_rect((*self.handle).getBounds.unwrap()(sprite)) }
    }

    /// Moves the given sprite to x, y and resets its bounds based on the bitmap dimensions and center.
    pub(crate) fn move_to(&self, sprite: *mut sys::LCDSprite, x: f32, y: f32) {
        unsafe {
            (*self.handle).moveTo.unwrap()(sprite, x, y);
        }
    }

    /// Moves the given sprite to by offsetting its current position by dx, dy.
    pub(crate) fn move_by(&self, sprite: *mut sys::LCDSprite, dx: f32, dy: f32) {
        unsafe {
            (*self.handle).moveBy.unwrap()(sprite, dx, dy);
        }
    }

    /// Sets the given sprite's image to the given bitmap.
    pub(crate) fn set_image(
        &self,
        sprite: *mut sys::LCDSprite,
        image: *mut sys::LCDBitmap,
        flip: sys::LCDBitmapFlip,
    ) {
        unsafe {
            (*self.handle).setImage.unwrap()(sprite, image, flip);
        }
    }

    /// Returns the LCDBitmap currently assigned to the given sprite.
    pub(crate) fn get_image(&self, sprite: *mut sys::LCDSprite) -> *mut sys::LCDBitmap {
        unsafe { (*self.handle).getImage.unwrap()(sprite) }
    }

    /// Sets the size. The size is used to set the sprite’s bounds when calling moveTo().
    pub(crate) fn set_size(&self, sprite: *mut sys::LCDSprite, width: f32, height: f32) {
        unsafe {
            (*self.handle).setSize.unwrap()(sprite, width, height);
        }
    }

    /// Sets the Z order of the given sprite. Higher Z sprites are drawn on top of those with lower Z order.
    pub(crate) fn set_z_index(&self, sprite: *mut sys::LCDSprite, z_index: i16) {
        unsafe {
            (*self.handle).setZIndex.unwrap()(sprite, z_index);
        }
    }

    /// Returns the Z index of the given sprite.
    pub(crate) fn get_z_index(&self, sprite: *mut sys::LCDSprite) -> i16 {
        unsafe { (*self.handle).getZIndex.unwrap()(sprite) }
    }

    /// Sets the mode for drawing the sprite’s bitmap.
    pub(crate) fn set_draw_mode(&self, sprite: *mut sys::LCDSprite, mode: sys::LCDBitmapDrawMode) {
        unsafe {
            (*self.handle).setDrawMode.unwrap()(sprite, mode);
        }
    }

    /// Flips the bitmap.
    pub(crate) fn set_image_flip(&self, sprite: *mut sys::LCDSprite, flip: sys::LCDBitmapFlip) {
        unsafe {
            (*self.handle).setImageFlip.unwrap()(sprite, flip);
        }
    }

    /// Returns the flip setting of the sprite’s bitmap.
    pub(crate) fn get_image_flip(&self, sprite: *mut sys::LCDSprite) -> sys::LCDBitmapFlip {
        unsafe { (*self.handle).getImageFlip.unwrap()(sprite) }
    }

    /// Specifies a stencil image to be set on the frame buffer before the sprite is drawn.
    pub(crate) fn set_stencil(&self, sprite: *mut sys::LCDSprite, stencil: *mut sys::LCDBitmap) {
        unsafe {
            (*self.handle).setStencil.unwrap()(sprite, stencil);
        }
    }

    /// Sets the clipping rectangle for sprite drawing.
    pub(crate) fn set_clip_rect(&self, sprite: *mut sys::LCDSprite, clip_rect: SideOffsets2D<i32>) {
        unsafe {
            (*self.handle).setClipRect.unwrap()(sprite, so2d_to_lcdrect(clip_rect));
        }
    }

    /// Clears the sprite’s clipping rectangle.
    pub(crate) fn clear_clip_rect(&self, sprite: *mut sys::LCDSprite) {
        unsafe {
            (*self.handle).clearClipRect.unwrap()(sprite);
        }
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

    /// Set the updatesEnabled flag of the given sprite (determines whether the sprite has its update function called). One is true, 0 is false.
    pub(crate) fn set_updates_enabled(&self, sprite: *mut sys::LCDSprite, flag: i32) {
        unsafe {
            (*self.handle).setUpdatesEnabled.unwrap()(sprite, flag);
        }
    }

    /// Get the updatesEnabled flag of the given sprite.
    pub(crate) fn updates_enabled(&self, sprite: *mut sys::LCDSprite) -> i32 {
        unsafe { (*self.handle).updatesEnabled.unwrap()(sprite) }
    }

    /// Set the collisionsEnabled flag of the given sprite (along with the collideRect, this determines whether the sprite participates in collisions). One is true, 0 is false. Set to 1 by default.
    pub(crate) fn set_collisions_enabled(&self, sprite: *mut sys::LCDSprite, flag: i32) {
        unsafe {
            (*self.handle).setCollisionsEnabled.unwrap()(sprite, flag);
        }
    }

    /// Get the collisionsEnabled flag of the given sprite.
    pub(crate) fn collisions_enabled(&self, sprite: *mut sys::LCDSprite) -> i32 {
        unsafe { (*self.handle).collisionsEnabled.unwrap()(sprite) }
    }

    /// Set the visible flag of the given sprite (determines whether the sprite has its draw function called). One is true, 0 is false.
    pub(crate) fn set_visible(&self, sprite: *mut sys::LCDSprite, flag: i32) {
        unsafe {
            (*self.handle).setVisible.unwrap()(sprite, flag);
        }
    }

    /// Get the visible flag of the given sprite.
    pub(crate) fn is_visible(&self, sprite: *mut sys::LCDSprite) -> i32 {
        unsafe { (*self.handle).isVisible.unwrap()(sprite) }
    }

    /// Marking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite, since it will be overdrawn anyway. If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag.
    pub(crate) fn set_opaque(&self, sprite: *mut sys::LCDSprite, flag: i32) {
        unsafe {
            (*self.handle).setOpaque.unwrap()(sprite, flag);
        }
    }

    /// Forces the given sprite to redraw.
    pub(crate) fn mark_dirty(&self, sprite: *mut sys::LCDSprite) {
        unsafe {
            (*self.handle).markDirty.unwrap()(sprite);
        }
    }

    /// Sets the tag of the given sprite. This can be useful for identifying sprites or types of sprites when using the collision API.
    pub(crate) fn set_tag(&self, sprite: *mut sys::LCDSprite, tag: u8) {
        unsafe {
            (*self.handle).setTag.unwrap()(sprite, tag);
        }
    }

    /// Returns the tag of the given sprite.
    pub(crate) fn get_tag(&self, sprite: *mut sys::LCDSprite) -> u8 {
        unsafe { (*self.handle).getTag.unwrap()(sprite) }
    }

    /// When flag is set to 1, the sprite will draw in screen coordinates, ignoring the currently-set drawOffset.
    ///
    /// This only affects drawing, and should not be used on sprites being used for collisions, which will still happen in world-space.
    pub(crate) fn set_ignores_draw_offset(&self, sprite: *mut sys::LCDSprite, flag: i32) {
        unsafe {
            (*self.handle).setIgnoresDrawOffset.unwrap()(sprite, flag);
        }
    }

    /// Sets the update function for the given sprite.
    pub(crate) fn set_update_function(
        &self,
        sprite: *mut sys::LCDSprite,
        func: sys::LCDSpriteUpdateFunction,
    ) {
        unsafe {
            (*self.handle).setUpdateFunction.unwrap()(sprite, func);
        }
    }

    /// Sets the draw function for the given sprite.
    pub(crate) fn set_draw_function(
        &self,
        sprite: *mut sys::LCDSprite,
        func: sys::LCDSpriteDrawFunction,
    ) {
        unsafe {
            (*self.handle).setDrawFunction.unwrap()(sprite, func);
        }
    }

    /// Sets x and y to the current position of sprite.
    pub(crate) fn get_position(&self, sprite: *mut sys::LCDSprite) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            (*self.handle).getPosition.unwrap()(sprite, &mut x, &mut y);
        }
        (x, y)
    }

    /// Frees and reallocates internal collision data, resetting everything to its default state.
    pub fn reset_collision_world(&self) {
        unsafe {
            (*self.handle).resetCollisionWorld.unwrap()();
        }
    }

    /// Marks the area of the given sprite, relative to its bounds, to be checked for collisions with other sprites' collide rects.
    pub(crate) fn set_collide_rect(&self, sprite: *mut sys::LCDSprite, collide_rect: Rect<f32>) {
        unsafe {
            (*self.handle).setCollideRect.unwrap()(sprite, rect_to_pdrect(collide_rect));
        }
    }

    /// Returns the given sprite’s collide rect.
    pub(crate) fn get_collide_rect(&self, sprite: *mut sys::LCDSprite) -> Rect<f32> {
        unsafe { pdrect_to_rect((*self.handle).getCollideRect.unwrap()(sprite)) }
    }

    /// Clears the given sprite’s collide rect.
    pub(crate) fn clear_collide_rect(&self, sprite: *mut sys::LCDSprite) {
        unsafe {
            (*self.handle).clearCollideRect.unwrap()(sprite);
        }
    }

    /// Set a callback that returns a SpriteCollisionResponseType for a collision between sprite and other.
    pub(crate) fn set_collision_response_function(
        &self,
        sprite: *mut sys::LCDSprite,
        func: sys::LCDSpriteCollisionFilterProc,
    ) {
        unsafe {
            (*self.handle).setCollisionResponseFunction.unwrap()(sprite, func);
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

    /// Sets the sprite’s stencil to the given pattern.
    pub(crate) fn set_stencil_pattern(&self, sprite: *mut sys::LCDSprite, pattern: *mut u8) {
        unsafe {
            (*self.handle).setStencilPattern.unwrap()(sprite, pattern);
        }
    }

    /// Clears the sprite’s stencil.
    pub(crate) fn clear_stencil(&self, sprite: *mut sys::LCDSprite) {
        unsafe {
            (*self.handle).clearStencil.unwrap()(sprite);
        }
    }

    /// Sets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data.
    pub(crate) fn set_userdata(
        &self,
        sprite: *mut sys::LCDSprite,
        userdata: *mut core::ffi::c_void,
    ) {
        unsafe {
            (*self.handle).setUserdata.unwrap()(sprite, userdata);
        }
    }

    /// Gets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data.
    pub(crate) fn get_userdata(&self, sprite: *mut sys::LCDSprite) -> *mut core::ffi::c_void {
        unsafe { (*self.handle).getUserdata.unwrap()(sprite) }
    }

    /// Specifies a stencil image to be set on the frame buffer before the sprite is drawn. If tile is set, the stencil will be tiled. Tiled stencils must have width evenly divisible by 32.
    pub(crate) fn set_stencil_image(
        &self,
        sprite: *mut sys::LCDSprite,
        stencil: *mut sys::LCDBitmap,
        tile: i32,
    ) {
        unsafe {
            (*self.handle).setStencilImage.unwrap()(sprite, stencil, tile);
        }
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

impl Sprite {
    pub(crate) fn from(handle: *mut sys::LCDSprite) -> Self {
        Self { handle }
    }

    pub(crate) fn from_ref<'a>(handle: *mut sys::LCDSprite) -> Ref<'a, Self> {
        Ref::from(Self { handle })
    }

    /// Allocates and returns a new Sprite.
    pub fn new() -> Self {
        PLAYDATE.sprite.new_sprite()
    }

    /// Sets x and y to the current position of sprite.
    pub fn get_position(&self) -> (f32, f32) {
        PLAYDATE.sprite.get_position(self.handle)
    }

    /// Sets the bounds of the given sprite with bounds.
    pub fn set_bounds(&self, bounds: Rect<f32>) {
        PLAYDATE.sprite.set_bounds(self.handle, bounds);
    }

    /// Returns the bounds of the given sprite as an PDRect;
    pub fn get_bounds(&self) -> Rect<f32> {
        PLAYDATE.sprite.get_bounds(self.handle)
    }

    /// Moves the given sprite to x, y and resets its bounds based on the bitmap dimensions and center.
    pub fn move_to(&self, x: f32, y: f32) {
        PLAYDATE.sprite.move_to(self.handle, x, y);
    }

    /// Moves the given sprite to by offsetting its current position by dx, dy.
    pub fn move_by(&self, dx: f32, dy: f32) {
        PLAYDATE.sprite.move_by(self.handle, dx, dy);
    }

    /// Sets the given sprite's image to the given bitmap.
    pub fn set_image(&self, image: impl AsRef<Bitmap>, flip: sys::LCDBitmapFlip) {
        PLAYDATE
            .sprite
            .set_image(self.handle, image.as_ref().handle, flip);
    }

    /// Returns the LCDBitmap currently assigned to the given sprite.
    pub fn get_image(&self) -> Ref<Bitmap> {
        Bitmap::from_ref(PLAYDATE.sprite.get_image(self.handle))
    }

    /// Sets the size. The size is used to set the sprite’s bounds when calling moveTo().
    pub fn set_size(&self, width: f32, height: f32) {
        PLAYDATE.sprite.set_size(self.handle, width, height);
    }

    /// Sets the Z order of the given sprite. Higher Z sprites are drawn on top of those with lower Z order.
    pub fn set_z_index(&self, z_index: i16) {
        PLAYDATE.sprite.set_z_index(self.handle, z_index);
    }

    /// Returns the Z index of the given sprite.
    pub fn get_z_index(&self) -> i16 {
        PLAYDATE.sprite.get_z_index(self.handle)
    }

    /// Sets the mode for drawing the sprite’s bitmap.
    pub fn set_draw_mode(&self, mode: sys::LCDBitmapDrawMode) {
        PLAYDATE.sprite.set_draw_mode(self.handle, mode);
    }

    /// Flips the bitmap.
    pub fn set_image_flip(&self, flip: sys::LCDBitmapFlip) {
        PLAYDATE.sprite.set_image_flip(self.handle, flip);
    }

    /// Returns the flip setting of the sprite’s bitmap.
    pub fn get_image_flip(&self) -> sys::LCDBitmapFlip {
        PLAYDATE.sprite.get_image_flip(self.handle)
    }

    /// Specifies a stencil image to be set on the frame buffer before the sprite is drawn.
    pub fn set_stencil(&self, stencil: impl AsRef<Bitmap>) {
        PLAYDATE
            .sprite
            .set_stencil(self.handle, stencil.as_ref().handle);
    }

    /// Sets the clipping rectangle for sprite drawing.
    pub fn set_clip_rect(&self, clip_rect: SideOffsets2D<i32>) {
        PLAYDATE.sprite.set_clip_rect(self.handle, clip_rect);
    }

    /// Clears the sprite’s clipping rectangle.
    pub fn clear_clip_rect(&self) {
        PLAYDATE.sprite.clear_clip_rect(self.handle);
    }

    /// Set the updatesEnabled flag of the given sprite (determines whether the sprite has its update function called).
    pub fn set_updates_enabled(&self, flag: bool) {
        PLAYDATE.sprite.set_updates_enabled(self.handle, flag as _);
    }

    /// Get the updatesEnabled flag of the given sprite.
    pub fn updates_enabled(&self) -> bool {
        PLAYDATE.sprite.updates_enabled(self.handle) == 1
    }

    /// Set the collisionsEnabled flag of the given sprite (along with the collideRect, this determines whether the sprite participates in collisions). Set to true by default.
    pub fn set_collisions_enabled(&self, flag: bool) {
        PLAYDATE
            .sprite
            .set_collisions_enabled(self.handle, flag as _);
    }

    /// Get the collisionsEnabled flag of the given sprite.
    pub fn collisions_enabled(&self) -> bool {
        PLAYDATE.sprite.collisions_enabled(self.handle) == 1
    }

    /// Set the visible flag of the given sprite (determines whether the sprite has its draw function called).
    pub fn set_visible(&self, flag: bool) {
        PLAYDATE.sprite.set_visible(self.handle, flag as _);
    }

    /// Get the visible flag of the given sprite.
    pub fn is_visible(&self) -> bool {
        PLAYDATE.sprite.is_visible(self.handle) == 1
    }

    /// Marking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite, since it will be overdrawn anyway. If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag.
    pub fn set_opaque(&self, flag: bool) {
        PLAYDATE.sprite.set_opaque(self.handle, flag as _);
    }

    /// Forces the given sprite to redraw.
    pub fn mark_dirty(&self) {
        PLAYDATE.sprite.mark_dirty(self.handle);
    }

    /// Sets the tag of the given sprite. This can be useful for identifying sprites or types of sprites when using the collision API.
    pub fn set_tag(&self, tag: u8) {
        PLAYDATE.sprite.set_tag(self.handle, tag);
    }

    /// Returns the tag of the given sprite.
    pub fn get_tag(&self) -> u8 {
        PLAYDATE.sprite.get_tag(self.handle)
    }

    /// When flag is set to 1, the sprite will draw in screen coordinates, ignoring the currently-set drawOffset.
    ///
    /// This only affects drawing, and should not be used on sprites being used for collisions, which will still happen in world-space.
    pub fn set_ignores_draw_offset(&self, flag: i32) {
        PLAYDATE.sprite.set_ignores_draw_offset(self.handle, flag);
    }

    /// Sets the update function for the given sprite.
    pub fn set_update_function(&self, func: sys::LCDSpriteUpdateFunction) {
        PLAYDATE.sprite.set_update_function(self.handle, func);
    }

    /// Sets the draw function for the given sprite.
    pub fn set_draw_function(&self, func: sys::LCDSpriteDrawFunction) {
        PLAYDATE.sprite.set_draw_function(self.handle, func);
    }

    /// Marks the area of the given sprite, relative to its bounds, to be checked for collisions with other sprites' collide rects.
    pub fn set_collide_rect(&self, collide_rect: Rect<f32>) {
        PLAYDATE.sprite.set_collide_rect(self.handle, collide_rect);
    }

    /// Returns the given sprite’s collide rect.
    pub fn get_collide_rect(&self) -> Rect<f32> {
        PLAYDATE.sprite.get_collide_rect(self.handle)
    }

    /// Clears the given sprite’s collide rect.
    pub fn clear_collide_rect(&self) {
        PLAYDATE.sprite.clear_collide_rect(self.handle);
    }

    /// Set a callback that returns a SpriteCollisionResponseType for a collision between sprite and other.
    pub fn set_collision_response_function(&self, func: sys::LCDSpriteCollisionFilterProc) {
        PLAYDATE
            .sprite
            .set_collision_response_function(self.handle, func);
    }

    /// Sets the sprite’s stencil to the given pattern.
    pub fn set_stencil_pattern(&self, pattern: *mut u8) {
        PLAYDATE.sprite.set_stencil_pattern(self.handle, pattern);
    }

    /// Clears the sprite’s stencil.
    pub fn clear_stencil(&self) {
        PLAYDATE.sprite.clear_stencil(self.handle);
    }

    /// Sets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data.
    pub fn set_userdata(&self, userdata: *mut core::ffi::c_void) {
        PLAYDATE.sprite.set_userdata(self.handle, userdata);
    }

    /// Gets the sprite’s userdata, an arbitrary pointer used for associating the sprite with other data.
    pub fn get_userdata(&self) -> *mut core::ffi::c_void {
        PLAYDATE.sprite.get_userdata(self.handle)
    }

    /// Specifies a stencil image to be set on the frame buffer before the sprite is drawn. If tile is set, the stencil will be tiled. Tiled stencils must have width evenly divisible by 32.
    pub fn set_stencil_image(&self, stencil: impl AsRef<Bitmap>, tile: i32) {
        PLAYDATE
            .sprite
            .set_stencil_image(self.handle, stencil.as_ref().handle, tile);
    }
}

impl AsRef<Self> for Sprite {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        PLAYDATE.sprite.free_sprite(self.handle as *mut _);
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        let handle = PLAYDATE.sprite.copy(self.handle as *mut _);
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
