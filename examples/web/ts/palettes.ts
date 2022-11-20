export const PALETTES = [
    {
        name: "greenish",
        colors: [
            [0x50, 0xcb, 0x93],
            [0x1b, 0x1a, 0x17]
        ]
    },
    {
        name: "b&w",
        colors: [
            [0xff, 0xff, 0xff],
            [0x1b, 0x1a, 0x17]
        ]
    },
    {
        name: "redish",
        colors: [
            [0xff, 0x00, 0x00],
            [0x1b, 0x1a, 0x17]
        ]
    }
];

export const PALETTES_MAP = Object.fromEntries(
    PALETTES.map((v) => [v.name, v])
);
