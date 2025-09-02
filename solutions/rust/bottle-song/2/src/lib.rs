static SONGS: [&str; 10] = [
    "Ten green bottles hanging on the wall,\nTen green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be nine green bottles hanging on the wall.",
    "Nine green bottles hanging on the wall,\nNine green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be eight green bottles hanging on the wall.",
    "Eight green bottles hanging on the wall,\nEight green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be seven green bottles hanging on the wall.",
    "Seven green bottles hanging on the wall,\nSeven green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be six green bottles hanging on the wall.",
    "Six green bottles hanging on the wall,\nSix green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be five green bottles hanging on the wall.",
    "Five green bottles hanging on the wall,\nFive green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be four green bottles hanging on the wall.",
    "Four green bottles hanging on the wall,\nFour green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be three green bottles hanging on the wall.",
    "Three green bottles hanging on the wall,\nThree green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be two green bottles hanging on the wall.",
    "Two green bottles hanging on the wall,\nTwo green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be one green bottle hanging on the wall.",
    "One green bottle hanging on the wall,\nOne green bottle hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be no green bottles hanging on the wall.",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let start = (10 - start_bottles) as usize;
    let end = start + take_down as usize;

    SONGS[start..end].join("\n\n")
}
