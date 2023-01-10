struct Token<T> {
    value: T,
    token_type: Tokens,
}

enum Tokens {

    // The token that implies a comment. Default is ';'.
    Comment,

    // The token that opens a section. Standard is `[`.
    SectionOpen,

    // The token that closes a section. Standard is `]`.
    SectionClose,

    // The token that maps a key to a value. Default is `=`.
    MapsTo,
}
