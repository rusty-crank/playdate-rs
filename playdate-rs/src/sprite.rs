pub struct Sprite {
    #[allow(unused)]
    handle: *const sys::playdate_sprite,
}

impl Sprite {
    pub(crate) fn new(handle: *const sys::playdate_sprite) -> Self {
        Self { handle }
    }

    // pub setAlwaysRedraw: ::core::option::Option<unsafe extern "C" fn(flag: ::core::ffi::c_int)>,
    // pub addDirtyRect: ::core::option::Option<unsafe extern "C" fn(dirtyRect: LCDRect)>,
    // pub drawSprites: ::core::option::Option<unsafe extern "C" fn()>,
    // pub updateAndDrawSprites: ::core::option::Option<unsafe extern "C" fn()>,
    // pub newSprite: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDSprite>,
    // pub freeSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub copy:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut LCDSprite>,
    // pub addSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub removeSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub removeSprites: ::core::option::Option<
    //     unsafe extern "C" fn(sprites: *mut *mut LCDSprite, count: ::core::ffi::c_int),
    // >,
    // pub removeAllSprites: ::core::option::Option<unsafe extern "C" fn()>,
    // pub getSpriteCount: ::core::option::Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    // pub setBounds:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, bounds: PDRect)>,
    // pub getBounds: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> PDRect>,
    // pub moveTo:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, x: f32, y: f32)>,
    // pub moveBy:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, dx: f32, dy: f32)>,
    // pub setImage: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, image: *mut LCDBitmap, flip: LCDBitmapFlip),
    // >,
    // pub getImage:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut LCDBitmap>,
    // pub setSize:
    //     ::core::option::Option<unsafe extern "C" fn(s: *mut LCDSprite, width: f32, height: f32)>,
    // pub setZIndex:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, zIndex: i16)>,
    // pub getZIndex: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> i16>,
    // pub setDrawMode: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, mode: LCDBitmapDrawMode),
    // >,
    // pub setImageFlip:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, flip: LCDBitmapFlip)>,
    // pub getImageFlip:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> LCDBitmapFlip>,
    // pub setStencil: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, stencil: *mut LCDBitmap),
    // >,
    // pub setClipRect:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, clipRect: LCDRect)>,
    // pub clearClipRect: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub setClipRectsInRange: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         clipRect: LCDRect,
    //         startZ: ::core::ffi::c_int,
    //         endZ: ::core::ffi::c_int,
    //     ),
    // >,
    // pub clearClipRectsInRange: ::core::option::Option<
    //     unsafe extern "C" fn(startZ: ::core::ffi::c_int, endZ: ::core::ffi::c_int),
    // >,
    // pub setUpdatesEnabled: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, flag: ::core::ffi::c_int),
    // >,
    // pub updatesEnabled:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> ::core::ffi::c_int>,
    // pub setCollisionsEnabled: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, flag: ::core::ffi::c_int),
    // >,
    // pub collisionsEnabled:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> ::core::ffi::c_int>,
    // pub setVisible: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, flag: ::core::ffi::c_int),
    // >,
    // pub isVisible:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> ::core::ffi::c_int>,
    // pub setOpaque: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, flag: ::core::ffi::c_int),
    // >,
    // pub markDirty: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub setTag: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, tag: u8)>,
    // pub getTag: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> u8>,
    // pub setIgnoresDrawOffset: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, flag: ::core::ffi::c_int),
    // >,
    // pub setUpdateFunction: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteUpdateFunction),
    // >,
    // pub setDrawFunction: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteDrawFunction),
    // >,
    // pub getPosition: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, x: *mut f32, y: *mut f32),
    // >,
    // pub resetCollisionWorld: ::core::option::Option<unsafe extern "C" fn()>,
    // pub setCollideRect:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, collideRect: PDRect)>,
    // pub getCollideRect:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite) -> PDRect>,
    // pub clearCollideRect: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub setCollisionResponseFunction: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteCollisionFilterProc),
    // >,
    // pub checkCollisions: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         sprite: *mut LCDSprite,
    //         goalX: f32,
    //         goalY: f32,
    //         actualX: *mut f32,
    //         actualY: *mut f32,
    //         len: *mut ::core::ffi::c_int,
    //     ) -> *mut SpriteCollisionInfo,
    // >,
    // pub moveWithCollisions: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         sprite: *mut LCDSprite,
    //         goalX: f32,
    //         goalY: f32,
    //         actualX: *mut f32,
    //         actualY: *mut f32,
    //         len: *mut ::core::ffi::c_int,
    //     ) -> *mut SpriteCollisionInfo,
    // >,
    // pub querySpritesAtPoint: ::core::option::Option<
    //     unsafe extern "C" fn(x: f32, y: f32, len: *mut ::core::ffi::c_int) -> *mut *mut LCDSprite,
    // >,
    // pub querySpritesInRect: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x: f32,
    //         y: f32,
    //         width: f32,
    //         height: f32,
    //         len: *mut ::core::ffi::c_int,
    //     ) -> *mut *mut LCDSprite,
    // >,
    // pub querySpritesAlongLine: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x1: f32,
    //         y1: f32,
    //         x2: f32,
    //         y2: f32,
    //         len: *mut ::core::ffi::c_int,
    //     ) -> *mut *mut LCDSprite,
    // >,
    // pub querySpriteInfoAlongLine: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x1: f32,
    //         y1: f32,
    //         x2: f32,
    //         y2: f32,
    //         len: *mut ::core::ffi::c_int,
    //     ) -> *mut SpriteQueryInfo,
    // >,
    // pub overlappingSprites: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         sprite: *mut LCDSprite,
    //         len: *mut ::core::ffi::c_int,
    //     ) -> *mut *mut LCDSprite,
    // >,
    // pub allOverlappingSprites: ::core::option::Option<
    //     unsafe extern "C" fn(len: *mut ::core::ffi::c_int) -> *mut *mut LCDSprite,
    // >,
    // pub setStencilPattern:
    //     ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite, pattern: *mut u8)>,
    // pub clearStencil: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub setUserdata: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite, userdata: *mut ::core::ffi::c_void),
    // >,
    // pub getUserdata: ::core::option::Option<
    //     unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut ::core::ffi::c_void,
    // >,
    // pub setStencilImage: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         sprite: *mut LCDSprite,
    //         stencil: *mut LCDBitmap,
    //         tile: ::core::ffi::c_int,
    //     ),
    // >,
}
