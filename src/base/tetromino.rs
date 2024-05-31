use crate::{
    base::point::{Offset, Point, PointState},
    const_vals::{NUM_PLAYFIELD_COLS, NUM_PLAYFIELD_ROWS, PIECE_INIT_POINT},
    fields::play,
};

const NUM_STATES: usize = 4;
const NUM_DATAS: usize = 4;
/// Represent partial tetromino datas except zero offset (which x = 0, y = 0)
pub(crate) type PartData = [Offset; 3];
pub(crate) type TetroDatas = [PartData; NUM_STATES];
pub(crate) type TetroKickOffsets = [Offset; 5];
pub(crate) type KickOffsets = [TetroKickOffsets; 4];

const I_DATAS: TetroDatas = [
    [Offset::new(-1, 0), Offset::new(1, 0), Offset::new(2, 0)],
    [Offset::new(0, 1), Offset::new(0, -1), Offset::new(0, -2)],
    [Offset::new(-2, 0), Offset::new(-1, 0), Offset::new(1, 0)],
    [Offset::new(0, 2), Offset::new(0, 1), Offset::new(0, -1)],
];
const J_DATAS: TetroDatas = [
    [Offset::new(-1, 1), Offset::new(-1, 0), Offset::new(1, 0)],
    [Offset::new(1, 1), Offset::new(0, 1), Offset::new(0, -1)],
    [Offset::new(-1, 0), Offset::new(1, 0), Offset::new(1, -1)],
    [Offset::new(0, 1), Offset::new(-1, -1), Offset::new(0, -1)],
];
const L_DATAS: TetroDatas = [
    [Offset::new(-1, 0), Offset::new(1, 0), Offset::new(1, 1)],
    [Offset::new(0, 1), Offset::new(0, -1), Offset::new(1, -1)],
    [Offset::new(-1, -1), Offset::new(-1, 0), Offset::new(1, 0)],
    [Offset::new(-1, 1), Offset::new(0, 1), Offset::new(0, -1)],
];
const O_DATAS: TetroDatas = [
    [Offset::new(0, 1), Offset::new(1, 1), Offset::new(1, 0)],
    [Offset::new(0, -1), Offset::new(1, 0), Offset::new(1, -1)],
    [Offset::new(-1, -1), Offset::new(-1, 0), Offset::new(0, -1)],
    [Offset::new(-1, 1), Offset::new(-1, 0), Offset::new(0, 1)],
];
const S_DATAS: TetroDatas = [
    [Offset::new(-1, 0), Offset::new(0, 1), Offset::new(1, 1)],
    [Offset::new(0, 1), Offset::new(1, 0), Offset::new(1, -1)],
    [Offset::new(-1, -1), Offset::new(1, 0), Offset::new(0, -1)],
    [Offset::new(-1, 1), Offset::new(-1, 0), Offset::new(0, -1)],
];
const T_DATAS: TetroDatas = [
    [Offset::new(-1, 0), Offset::new(0, 1), Offset::new(1, 0)],
    [Offset::new(0, 1), Offset::new(1, 0), Offset::new(0, -1)],
    [Offset::new(-1, 0), Offset::new(1, 0), Offset::new(0, -1)],
    [Offset::new(-1, 0), Offset::new(0, 1), Offset::new(0, -1)],
];
const Z_DATAS: TetroDatas = [
    [Offset::new(-1, 1), Offset::new(0, 1), Offset::new(1, 0)],
    [Offset::new(1, 0), Offset::new(1, 1), Offset::new(0, -1)],
    [Offset::new(-1, 0), Offset::new(0, -1), Offset::new(1, -1)],
    [Offset::new(-1, -1), Offset::new(-1, 0), Offset::new(0, 1)],
];
const ZERO_OFFSET: Offset = Offset {
    row_offset: 0,
    col_offset: 0,
};

const ZERO_TETRO_OFFSET: TetroKickOffsets = [
    ZERO_OFFSET,
    ZERO_OFFSET,
    ZERO_OFFSET,
    ZERO_OFFSET,
    ZERO_OFFSET,
];

/// J,L,S,T,Z Tetromino wall kick offsets
const COMMON5_OFFSETS: KickOffsets = [
    ZERO_TETRO_OFFSET,
    [
        ZERO_OFFSET,
        Offset::new(1, 0),
        Offset::new(1, -1),
        Offset::new(0, 2),
        Offset::new(1, 2),
    ],
    ZERO_TETRO_OFFSET,
    [
        ZERO_OFFSET,
        Offset::new(-1, 0),
        Offset::new(-1, -1),
        Offset::new(0, 2),
        Offset::new(-1, 2),
    ],
];

/// I Tetromino wall kick offsets
const I_OFFSETS: KickOffsets = [
    [
        ZERO_OFFSET,
        Offset::new(-1, 0),
        Offset::new(2, 0),
        Offset::new(-1, 0),
        Offset::new(2, 0),
    ],
    [
        Offset::new(-1, 0),
        ZERO_OFFSET,
        ZERO_OFFSET,
        Offset::new(0, 1),
        Offset::new(0, -2),
    ],
    [
        Offset::new(-1, 1),
        Offset::new(1, 1),
        Offset::new(-2, 1),
        Offset::new(1, 0),
        Offset::new(-2, 0),
    ],
    [
        Offset::new(0, 1),
        Offset::new(0, 1),
        Offset::new(0, 1),
        Offset::new(0, -1),
        Offset::new(0, 2),
    ],
];

/// O tetromino wall kick offsets (**Important: this data type is not same to other kind tetromino**)
const O_OFFSETS: [[Offset; 1]; 4] = [
    [ZERO_OFFSET],
    [Offset::new(0, -1)],
    [Offset::new(-1, -1)],
    [Offset::new(-1, 0)],
];

impl<const NUM_OFFSETS: usize> std::ops::Index<TetroState> for [[Offset; NUM_OFFSETS]; 4] {
    type Output = [Offset];

    fn index(&self, index: TetroState) -> &Self::Output {
        unsafe { self.get_unchecked(index as usize) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TetrominoKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetrominoKind {
    pub(crate) fn color(&self) -> TetrominoColor {
        unsafe { *TetrominoColor::KIND_COLORS.get_unchecked(*self as usize) }
    }

    fn part_data(&self, state: TetroState) -> PartData {
        unsafe { *self.datas().get_unchecked(state as usize) }
    }

    fn datas(&self) -> &TetroDatas {
        match self {
            TetrominoKind::I => &I_DATAS,
            TetrominoKind::J => &J_DATAS,
            TetrominoKind::L => &L_DATAS,
            TetrominoKind::O => &O_DATAS,
            TetrominoKind::S => &S_DATAS,
            TetrominoKind::T => &T_DATAS,
            TetrominoKind::Z => &Z_DATAS,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum TetroState {
    #[default]
    Zero,
    Right,
    Second,
    Left,
}

impl TetroState {
    pub(crate) const RANGE: std::ops::Range<u8> = 0..4;
    const fn new() -> Self {
        Self::Zero
    }
}

impl From<u8> for TetroState {
    fn from(value: u8) -> Self {
        let mod_rst = value % 4;
        match mod_rst {
            0 => Self::Zero,
            1 => Self::Right,
            2 => Self::Second,
            3 => Self::Left,
            _ => panic!("This should not happed!!!"),
        }
    }
}

impl std::ops::Add<u8> for TetroState {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self::from(self as u8 + rhs)
    }
}

impl std::ops::Add<Self> for TetroState {
    type Output = Self;

    fn add(self, rhs: TetroState) -> Self::Output {
        self + rhs as u8
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TetrominoColor {
    Cyan = 14,
    Red = 9,
    Orange = 214,
    Yellow = 11,
    Green = 2,
    Blue = 12,
    Purple = 5,
    #[cfg(feature = "test_wallkick")]
    Gray = 8,
    White = 15,
    // Black = 0,
}

impl std::fmt::Display for TetrominoColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl TetrominoColor {
    const KIND_COLORS: [TetrominoColor; 7] = [
        TetrominoColor::Cyan,
        TetrominoColor::Blue,
        TetrominoColor::Orange,
        TetrominoColor::Green,
        TetrominoColor::Purple,
        TetrominoColor::Yellow,
        TetrominoColor::Red,
    ];
    pub(crate) fn to_id(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Tetromino {
    kind: TetrominoKind,
    state: TetroState,
}

impl Tetromino {
    pub(crate) const fn new(kind: TetrominoKind) -> Self {
        Self {
            kind,
            state: TetroState::new(),
        }
    }

    pub(crate) fn set_state(&mut self, new_state: TetroState) {
        self.state = new_state
    }

    pub(crate) fn state(&self) -> TetroState {
        self.state
    }

    pub(crate) fn shadow_point_state(&self) -> PointState {
        PointState::Shadow(251)
    }

    pub(crate) fn color_point_state(&self) -> PointState {
        PointState::Color(self.color())
    }

    pub(crate) fn unavailable_point_state(&self) -> PointState {
        PointState::Color(TetrominoColor::White)
    }

    pub(crate) fn data(&self) -> [Offset; NUM_DATAS] {
        let part_data = self.kind.part_data(self.state);
        [ZERO_OFFSET, part_data[0], part_data[1], part_data[2]]
    }

    pub(crate) fn color(&self) -> TetrominoColor {
        self.kind.color()
    }

    pub(crate) fn kick_offsets(&self) -> &[Offset] {
        match self.kind {
            TetrominoKind::J
            | TetrominoKind::L
            | TetrominoKind::S
            | TetrominoKind::T
            | TetrominoKind::Z => &COMMON5_OFFSETS[self.state],
            TetrominoKind::I => &I_OFFSETS[self.state],
            TetrominoKind::O => &O_OFFSETS[self.state],
        }
    }

    pub(crate) fn check_downable(&self) -> bool {
        self.check(PIECE_INIT_POINT, is_color_point)
    }

    pub(crate) fn check_swappable(&self, curr_piece_center: Point) -> bool {
        self.check(curr_piece_center, |point| {
            point.out_of_bound(
                NUM_PLAYFIELD_ROWS as i16 - 1,
                NUM_PLAYFIELD_COLS as i16 - 1,
            )
        })
    }

    #[inline(always)]
    fn check(&self, base_point: Point, predicate: fn(Point) -> bool) -> bool {
        for offset in self.data() {
            if predicate(base_point + offset) {
                return false;
            }
        }
        true
    }

    pub(crate) fn check_movable(&self, base_point: Point) -> bool {
        self.check(base_point, |point| {
            point.out_of_bound(
                NUM_PLAYFIELD_ROWS as i16 - 1,
                NUM_PLAYFIELD_COLS as i16 - 1,
            ) || is_color_point(point)
        })
    }

}

#[inline(always)]
fn is_color_point(point: Point) -> bool {
    matches!(play::playfield()[point], PointState::Color(..))
}
