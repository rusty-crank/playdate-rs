pub struct PlaydateScoreboards {
    #[allow(unused)]
    handle: *const sys::playdate_scoreboards,
}

impl PlaydateScoreboards {
    pub(crate) fn new(handle: *const sys::playdate_scoreboards) -> Self {
        Self { handle }
    }

    // pub addScore: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         boardId: *const ::core::ffi::c_char,
    //         value: u32,
    //         callback: AddScoreCallback,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub getPersonalBest: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         boardId: *const ::core::ffi::c_char,
    //         callback: PersonalBestCallback,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub freeScore: ::core::option::Option<unsafe extern "C" fn(score: *mut PDScore)>,
    // pub getScoreboards: ::core::option::Option<
    //     unsafe extern "C" fn(callback: BoardsListCallback) -> ::core::ffi::c_int,
    // >,
    // pub freeBoardsList: ::core::option::Option<unsafe extern "C" fn(boardsList: *mut PDBoardsList)>,
    // pub getScores: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         boardId: *const ::core::ffi::c_char,
    //         callback: ScoresCallback,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub freeScoresList: ::core::option::Option<unsafe extern "C" fn(scoresList: *mut PDScoresList)>,
}
