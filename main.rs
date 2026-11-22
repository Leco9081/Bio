// =====================================================================
//  BIO  -  FASE 1.5
//  Crafting + bancada + portas e CASAS (interior dobrado), arvores-sprite
//  que caem, mudas, 3 camadas no mundo (chao / piso / blocos), visual
//  "3D", skins (exportar/importar), cores/genero e menu de configuracoes.
//
//  Projeto com apenas 2 arquivos: Cargo.toml + src/main.rs
//  Nenhum arquivo externo no projeto: todos os sprites sao gerados na memoria
//  a partir de "pixel art em texto" (veja a SECAO 1).
//  (Em tempo de execucao o jogo cria 2 coisas sozinho: bio_config.txt e a
//   pasta skins/ com as skins que voce exportar.)
//
//  COMO EDITAR OS SPRITES
//  ----------------------
//  * Va ate a "SECAO 1 - ARTE" (logo abaixo). Cada caractere = 1 pixel.
//  * '.' (ponto) = transparente.
//  * A cor de cada letra esta na funcao `palette()`.
//  * Salve, rode `cargo run --release` e aperte F1 no jogo: a GALERIA DE
//    SPRITES mostra tudo ampliado e o jogador nas 8 direcoes.
//  * As cores do personagem (cabelo, pele, roupa...) NAO precisam ser
//    editadas no codigo: use o menu Personagem (Esc -> Personagem).
//
//  CONTROLES
//  ---------
//  WASD / Setas ... andar (8 direcoes)       Mouse ........ mirar
//  Botao esquerdo . bater / quebrar (segure) Botao direito  colocar / interagir
//  Shift + direito  colocar como PISO (camada de piso, acima do chao)
//  1-9 / roda ..... escolher slot            E ............ inventario + crafting
//  Q .............. soltar 1 item            + / - ........ zoom
//  Esc ............ menu (Personagem, Configuracoes...)
//  F1 galeria de sprites | F3 debug | F4 teste de dano
//  H .............. mostra / esconde a LISTA DE CONTROLES
//
//  CELULAR (ANDROID)
//  -----------------
//  O mesmo codigo roda no celular: la os controles viram toque (veja a
//  SECAO 13): joystick, BATER/USAR (arraste no botao para mirar), hotbar por
//  toque e botoes para TODAS as teclas (Menu, Inv, F1, F3, F4, Z+, Z-, Q, Piso).
//  Para gerar o APK veja o Cargo.toml ([package.metadata.android]) e o
//  arquivo .github/workflows/apk.yml (build automatico na nuvem).
//
//  MODO CELULAR (TESTE)  -  Esc -> Configuracoes -> "Modo celular (teste)"
//  ---------------------------------------------------------------------
//  Padrao: NAO. Quando ligado, aparecem na tela os botoes (joystick,
//  BATER, USAR, hotbar e os botoes das teclas: Esc, E, Q, F1, F3, F4, +, -,
//  Shift e H) para voce jogar so com cliques/toques, por exemplo num
//  emulador de Windows no Android (Winlator / GameHub). O MOUSE faz o
//  papel do dedo e o teclado continua funcionando. A MIRA e AUTOMATICA: o
//  bloco alvo fica sempre na direcao em que o personagem anda. Clicar no
//  mundo vira a mira para aquele lado. Com o modo DESLIGADO tudo continua
//  como antes (mouse mira, WASD anda). Veja a SECAO 14 no fim do arquivo.
//
//  ARQUIVOS DO JOGO: bio_config.txt e a pasta skins/ ficam SEMPRE na mesma
//  pasta do bio.exe (caminho relativo ao executavel), entao a pasta do jogo
//  pode ser movida/zipada para qualquer lugar.
// =====================================================================
#![allow(dead_code)]

use macroquad::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::f32::consts::PI;
use std::fs;

// =====================================================================
// SECAO 1 - ARTE (SPRITES)            <<<<<<  EDITE AQUI  >>>>>>
// =====================================================================

fn hex(v: u32) -> Color {
    Color::from_rgba((v >> 16) as u8, (v >> 8) as u8, v as u8, 255)
}

/// Letra -> cor. Qualquer letra que nao esteja aqui (inclusive '.') = transparente.
fn palette(c: char) -> Option<Color> {
    let v: u32 = match c {
        // --- geral
        'K' => 0x1b1320, // contorno escuro
        'W' => 0xf4f4f4, // branco
        // --- personagem
        'H' => 0x4b2e1b, // cabelo
        'h' => 0x6e4a2c, // cabelo (brilho)
        'S' => 0xf2b890, // pele
        's' => 0xd99468, // pele (sombra)
        'E' => 0x1a1a2e, // olho
        'B' => 0x3aa8bd, // camisa
        'b' => 0x277d90, // camisa (sombra)
        'P' => 0x3a4aa8, // calca
        'p' => 0x2b3680, // calca (sombra)
        'k' => 0x4a4a55, // sapato / cinza escuro
        // --- grama
        'g' => 0x5fae3b,
        'G' => 0x478f2d,
        'y' => 0x86cf5a,
        'L' => 0x2f6b24, // folha escura
        // --- terra
        'd' => 0x8b5a2b,
        'D' => 0x6a4122,
        't' => 0xa9743f,
        // --- areia
        'n' => 0xe8d68e,
        'N' => 0xcbb872,
        'e' => 0xf4e9b4,
        // --- neve
        'w' => 0xf1f6fa,
        'v' => 0xcfdcec,
        'V' => 0xb3c4dc,
        // --- agua
        'o' => 0x3b7dd8,
        'q' => 0x6fa8f0,
        'Q' => 0x2a5fb0,
        // --- pedra
        'r' => 0x8a8a92,
        'R' => 0x6a6a72,
        'z' => 0xa6a6ae,
        // --- madeira
        'f' => 0xb98a50,
        'F' => 0x94683a,
        'i' => 0xd1a468,
        // --- cacto
        'c' => 0x3f9a47,
        'C' => 0x2c7434,
        'u' => 0xf0f0c0, // espinhos
        // --- flor / coracao
        'M' => 0xe0405a, // vermelho
        'm' => 0xf2d33c, // amarelo
        // --- buraco
        'J' => 0x1c140f,
        'j' => 0x2c2119,
        _ => return None,
    };
    Some(hex(v))
}

// ---------------------------------------------------------------------
// JOGADOR (12 x 16 pixels). Desenhado em 5 vistas: Frente(S), Diagonal
// frente(SE), Lado(E), Diagonal costas(NE), Costas(N). As vistas da
// esquerda (SO, O, NO) sao o espelho automatico das da direita.
//
// Cada vista tem: cabeca (7 linhas), tronco (5 linhas), tronco com braco
// estendido = animacao de interagir (5 linhas). As pernas (4 linhas)
// tem 3 poses: parado, passo A, passo B.
// ---------------------------------------------------------------------

const HEAD_S: [&str; 7] = [
    "...KKKKKK...",
    "..KHHHHHHK..",
    ".KHHhHHhHHK.",
    ".KHSSSSSSHK.",
    ".KSSESSESSK.",
    ".KSSSSSSSSK.",
    "..KKSSSSKK..",
];
const TORSO_S: [&str; 5] = [
    "..KBSSSSBK..",
    ".KBBBBBBBBK.",
    ".KBBBBBBBBK.",
    ".KSBBbbBBSK.",
    "..KbbbbbbK..",
];
const POINT_S: [&str; 5] = [
    "..KBSSSSBK..",
    ".KBBBBBBBBK.",
    ".KBBBBBBBBBK",
    ".KSBBbbBBSSK",
    "..KbbbbbbK..",
];

const HEAD_SE: [&str; 7] = [
    "...KKKKKK...",
    "..KHHHHHHK..",
    ".KHHhHHhHHK.",
    ".KHHSSSSSSK.",
    ".KHSSESSESK.",
    ".KHSSSSSSSK.",
    "..KKSSSSKK..",
];
const TORSO_SE: [&str; 5] = [
    "..KBBSSSBK..",
    ".KBBBBBBBBK.",
    ".KBBBBBBBBK.",
    ".KSBBbbBBSK.",
    "..KbbbbbbK..",
];
const POINT_SE: [&str; 5] = [
    "..KBBSSSBK..",
    ".KBBBBBBBBBK",
    ".KBBBBBBBSSK",
    ".KSBBbbBBBK.",
    "..KbbbbbbK..",
];

const HEAD_E: [&str; 7] = [
    "...KKKKKK...",
    "..KHHHHHHK..",
    ".KHHhHHHHHK.",
    ".KHHHSSSSSK.",
    ".KHHSSSSESK.",
    ".KHHSSSSSSsK",
    "..KKSSSSKKK.",
];
const TORSO_E: [&str; 5] = [
    "..KBBSSBBK..",
    "..KBBBBBBK..",
    "..KBBbbBBK..",
    "..KBBSSBBK..",
    "..KbbbbbbK..",
];
const POINT_E: [&str; 5] = [
    "..KBBSSBBK..",
    "..KBBBBBBSSK",
    "..KBBbbBBBK.",
    "..KBBBBBBK..",
    "..KbbbbbbK..",
];

const HEAD_NE: [&str; 7] = [
    "...KKKKKK...",
    "..KHHHHHHK..",
    ".KHHhHHHHHK.",
    ".KHHHHHHHSK.",
    ".KHHHHHHHSK.",
    ".KHHHHHHHHK.",
    "..KKHHHHKK..",
];
const POINT_NE: [&str; 5] = [
    "..KBBBBBBBSK",
    ".KBBBBBBBBBK",
    ".KBBBBBBBBK.",
    ".KSBBbbBBBK.",
    "..KbbbbbbK..",
];

const HEAD_N: [&str; 7] = [
    "...KKKKKK...",
    "..KHHHHHHK..",
    ".KHHhHHhHHK.",
    ".KHHHHHHHHK.",
    ".KHHHhHHHHK.",
    ".KHHHHHHHHK.",
    "..KKHHHHKK..",
];
const TORSO_N: [&str; 5] = [
    "..KBBBBBBK..",
    ".KBBBBBBBBK.",
    ".KBBBBBBBBK.",
    ".KSBBbbBBSK.",
    "..KbbbbbbK..",
];
const POINT_N: [&str; 5] = [
    "..KBBBBBBK..",
    ".KBBBBBBBBSK",
    ".KBBBBBBBBBK",
    ".KSBBbbBBBK.",
    "..KbbbbbbK..",
];

// Pernas: [parado, passo A, passo B]
const LEGS_FRONT: [[&str; 4]; 3] = [
    [
        "..KPPPPPPK..",
        "..KPPKKPPK..",
        "..KkkKKkkK..",
        "..KKKKKKKK..",
    ],
    [
        "..KPPPPPPK..",
        "..KkkKKPPK..",
        "..KKKKKkkK..",
        "......KKKK..",
    ],
    [
        "..KPPPPPPK..",
        "..KPPKKkkK..",
        "..KkkKKKKK..",
        "..KKKK......",
    ],
];
const LEGS_SIDE: [[&str; 4]; 3] = [
    [
        "..KPPPPPPK..",
        "..KPPppPPK..",
        "..KPPPPkkkK.",
        "..KKKKKKKKK.",
    ],
    [
        "..KPPPPPPK..",
        "..KPPK.KPPK.",
        "..KkkK.KkkkK",
        "..KKKK.KKKKK",
    ],
    [
        "..KPPPPPPK..",
        "..KPPK.KPPK.",
        "..KkkkK.KkkK",
        "..KKKKK.KKKK",
    ],
];

struct HumanArt {
    head: [&'static str; 7],
    torso: [&'static str; 5],
    point: [&'static str; 5],
    legs: [[&'static str; 4]; 3],
}

/// Ordem das vistas: 0=S  1=SE  2=E  3=NE  4=N
fn player_views() -> [HumanArt; 5] {
    [
        HumanArt { head: HEAD_S, torso: TORSO_S, point: POINT_S, legs: LEGS_FRONT },
        HumanArt { head: HEAD_SE, torso: TORSO_SE, point: POINT_SE, legs: LEGS_FRONT },
        HumanArt { head: HEAD_E, torso: TORSO_E, point: POINT_E, legs: LEGS_SIDE },
        HumanArt { head: HEAD_NE, torso: TORSO_N, point: POINT_NE, legs: LEGS_FRONT },
        HumanArt { head: HEAD_N, torso: TORSO_N, point: POINT_N, legs: LEGS_FRONT },
    ]
}

// ---------------------------------------------------------------------
// BLOCOS (16 x 16 pixels cada)
// ---------------------------------------------------------------------

const ART_GRASS: [&str; 16] = [
    "gggggggggggggggg",
    "ggyggggggGgggggg",
    "gggggGggggggyggg",
    "gGgggggggggggGgg",
    "ggggyggggggggggg",
    "gggggggGggggyggg",
    "gyggggggggggGggg",
    "ggggGgggggyggggG",
    "ggGggggggggggggg",
    "ggggggyggggGgggg",
    "ggggggggggyggggg",
    "gGggggggGggggGgg",
    "ggggyggggggggggg",
    "ggggggGggggygggg",
    "gygggggggggggGgg",
    "gggGggggyggggggg",
];

const ART_DIRT: [&str; 16] = [
    "dddddddddddddddd",
    "dtdddDdddddddddd",
    "ddddddddddtddDdd",
    "dDdddddddddddddd",
    "ddddddtdddddddDd",
    "dddddddddDdddddd",
    "dtdddddddddddtdd",
    "ddddDddddddddddd",
    "ddddddddddtddddd",
    "dDdddddddddddDdd",
    "ddddtddddddddddd",
    "ddddddddDddddddt",
    "dtdddddddddddddd",
    "ddddDdddddddtddd",
    "ddddddddddtddddd",
    "dDddddddddddddDd",
];

const ART_SAND: [&str; 16] = [
    "nnnnnnnnnnnnnnnn",
    "nnennnnnnNnnnnnn",
    "nnnnnnnnnnnnnnen",
    "nNnnnnnnnnnnnnnn",
    "nnnnnnennnnnnnNn",
    "nnnnnnnnnNnnnnnn",
    "nnennnnnnnnnnnnn",
    "nnnnnNnnnnnnnnen",
    "nnnnnnnnnnennnnn",
    "nNnnnnnnnnnnnNnn",
    "nnnnnnennnnnnnnn",
    "nnnnnnnnnNnnnnen",
    "nnennnnnnnnnnnnn",
    "nnnnnNnnnnnnnnNn",
    "nnnnnnnnnnennnnn",
    "nNnnnnnnnnnnnnnn",
];

const ART_SNOW: [&str; 16] = [
    "wwwwwwwwwwwwwwww",
    "wwvwwwwwwwwvwwww",
    "wwwwwwwVwwwwwwww",
    "wvwwwwwwwwwwwwvw",
    "wwwwwwvwwwwwwwww",
    "wwwwwwwwwVwwwwww",
    "wwvwwwwwwwwwwwwv",
    "wwwwwVwwwwwwwwww",
    "wwwwwwwwwwvwwwww",
    "wvwwwwwwwwwwwvww",
    "wwwwwwvwwwwwwwww",
    "wwwwwwwwwwwwwwVw",
    "wwvwwwwwwwvwwwww",
    "wwwwwVwwwwwwwwww",
    "wwwwwwwwwwvwwwww",
    "wvwwwwwwwwwwwwwv",
];

const ART_WATER: [&str; 16] = [
    "oooooooooooooooo",
    "ooqqqooooooooooo",
    "ooooooooooooooqq",
    "ooooooooooQooooo",
    "ooooqqqooooooooo",
    "oQooooooooooooqq",
    "ooooooooooqqqooo",
    "oooooooooooooooo",
    "ooqqoooooooooQoo",
    "ooooooooqqqooooo",
    "oooooQoooooooooo",
    "ooooooooooooooqq",
    "qqqooooooQoooooo",
    "oooooooooooooooo",
    "ooooooqqqooooooo",
    "oQoooooooooooooo",
];

// Buraco (chao quebrado). Na Fase 2 ele leva ao subterraneo.
const ART_HOLE: [&str; 16] = [
    "DDDDDDDDDDDDDDDD",
    "DJJJJJJJJJJJJJJD",
    "DJjjJJJJJJJJjjJD",
    "DJjJJJJJJJjJJJJD",
    "DJJJJJjjJJJJJJJD",
    "DJJJJJJJJJJJjJJD",
    "DJJjJJJJJjJJJJJD",
    "DJJJJJJJJJJJJJJD",
    "DJJJjJJJJJJJJjJD",
    "DJJJJJJJJJjjJJJD",
    "DJjJJJJJJJJJJJJD",
    "DJJJJJjJJJJJjJJD",
    "DJJJJJJJJJJJJJJD",
    "DJjjJJJJjJJJJJJD",
    "DJJJJJJJJJJJJJJD",
    "DDDDDDDDDDDDDDDD",
];

const ART_STONE: [&str; 16] = [
    "rrrrrrrrrrrrrrrr",
    "rrzrrrrrrRrrrrrr",
    "rrrrrzrrrrrrrrzr",
    "rRrrrrrrrrrrrrrr",
    "rrrrrrrRrrzrrrrr",
    "rrzrrrrrrrrrRrrr",
    "rrrrrRrrrrrrrrrr",
    "rrrrrrrrrzrrrrrz",
    "rzrrrrrrrrrrrrrr",
    "rrrrrrRrrrrrrzrr",
    "rrrrrrrrrrRrrrrr",
    "rRrrrzrrrrrrrrrr",
    "rrrrrrrrrrzrrRrr",
    "rrzrrrrrrrrrrrrr",
    "rrrrrRrrrrrrrzrr",
    "rRrrrrrrrrzrrrrr",
];

const ART_COBBLE: [&str; 16] = [
    "RRRRRRRRRRRRRRRR",
    "RzzrrrRzzzrrrrrR",
    "RzrrrrRzrrrrrrrR",
    "RrrrrrRrrrrrrrrR",
    "RrrrrrRrrrrrrrrR",
    "RRRRRRRRRRRRRRRR",
    "RzzzRzzrrrRzzrrR",
    "RzrrRzrrrrRzrrrR",
    "RrrrRrrrrrRrrrrR",
    "RrrrRrrrrrRrrrrR",
    "RRRRRRRRRRRRRRRR",
    "RzzzzrrrRzzzrrrR",
    "RzrrrrrrRzrrrrrR",
    "RrrrrrrrRrrrrrrR",
    "RrrrrrrrRrrrrrrR",
    "RRRRRRRRRRRRRRRR",
];

const ART_PLANKS: [&str; 16] = [
    "FFFFFFFFFFFFFFFF",
    "fiiffffFffffifff",
    "ffffiffFfffffiff",
    "fffffffFffiiffff",
    "FFFFFFFFFFFFFFFF",
    "ffiiffffffFfiiff",
    "ffffffifffFfffif",
    "iiffffffffFfffff",
    "FFFFFFFFFFFFFFFF",
    "fiiffffFffffifff",
    "ffffiffFfffffiff",
    "fffffffFffiiffff",
    "FFFFFFFFFFFFFFFF",
    "ffiiffffffFfiiff",
    "ffffffifffFfffif",
    "iiffffffffFfffff",
];

// Tronco visto de cima (aneis da madeira)
const ART_LOG: [&str; 16] = [
    "DDDDDDDDDDDDDDDD",
    "DddddddddddddddD",
    "DdffffffffffffdD",
    "DdfFFFFFFFFFFfdD",
    "DdfFiiiiiiiiFfdD",
    "DdfFiffffffiFfdD",
    "DdfFiffFFffiFfdD",
    "DdfFifFffFfiFfdD",
    "DdfFifFffFfiFfdD",
    "DdfFiffFFffiFfdD",
    "DdfFiffffffiFfdD",
    "DdfFiiiiiiiiFfdD",
    "DdfFFFFFFFFFFfdD",
    "DdffffffffffffdD",
    "DddddddddddddddD",
    "DDDDDDDDDDDDDDDD",
];

const ART_LEAVES: [&str; 16] = [
    "GgyGgGLGgyGgGLgy",
    "gyGgLGgyGgLGgyGg",
    "yGLgyggGyGgLggGy",
    "GgyGGLgygGygLGgG",
    "LgGygyGgLGgyGgyL",
    "gyGgGLgGygGLgyGg",
    "GLgygGygGgLGygGg",
    "gGyLgyGgyGgyLGgG",
    "yGgGLgyGgLGgyGgy",
    "gLGyGgyLGygGgLGy",
    "GgyGgLGgyGLygGgG",
    "yGgLygGyGgGyLgyG",
    "gGygLGgGygGgyLGg",
    "LgGygyLGgGygGyGL",
    "gyGgGgyGLgyLgGgy",
    "GgLGygGygGyGLgGy",
];

// Cacto: fundo transparente (a areia aparece em volta)
const ART_CACTUS: [&str; 16] = [
    "................",
    "................",
    "..KKKKKKKKKKKK..",
    "..KccccccccccK..",
    "..KcCcccucCccK..",
    "..KcccuccccccK..",
    "..KccccccCccuK..",
    "..KcuccccccCcK..",
    "..KccccucccccK..",
    "..KcCcccccuccK..",
    "..KccccCcccucK..",
    "..KcuccccccccK..",
    "..KCCCCCCCCCCK..",
    "..KKKKKKKKKKKK..",
    "................",
    "................",
];

const ART_TALLGRASS: [&str; 16] = [
    "................",
    "................",
    "................",
    "....y.......y...",
    "....g...y...gy..",
    "...ygy..g..ygG..",
    "...gGg.ygG.gGg.y",
    "..ygGgygGgygGgg.",
    "..gGGgggGGgggGG.",
    ".ygGGgygGGgygGGg",
    ".gGGGggGGGggGGGg",
    ".gGGGgGGGGgGGGG.",
    "..GGGGGGGGGGGG..",
    "...GGGGGGGGGG...",
    "................",
    "................",
];

const ART_FLOWER: [&str; 16] = [
    "................",
    "................",
    "................",
    ".....M..........",
    "....MmM.....M...",
    ".....M.....MmM..",
    ".....G......M...",
    ".....G......G...",
    "....GG......G...",
    "....G......GG...",
    "....G...G..G....",
    "...GG...GG.G....",
    "...G....G.......",
    "..GG....G.......",
    "................",
    "................",
];

const ART_SEEDS: [&str; 16] = [
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "....KKK....KKK..",
    "...KtddK..KtddK.",
    "...KdDDK..KdDDK.",
    "....KKK....KKK..",
    "......KKK.......",
    ".....KtddK......",
    ".....KdDDK......",
    "......KKK.......",
    "................",
    "................",
];

// ---------------------------------------------------------------------
// FERRAMENTAS: 'X' = cor principal do material, 'x' = sombra, 'Y' = brilho.
// A cor depende do material (madeira, pedra, ferro, diamante) - veja Tier::colors().
// O cabo usa 'd' e 'D'.
// ---------------------------------------------------------------------
const ART_PICKAXE: [&str; 16] = [
    "................",
    "....KKKKKKKK....",
    "..KKXYYYYXXxKK..",
    ".KXXXxKKKKxXXXK.",
    ".KXxK.KdDK.KXxK.",
    ".KKKK.KdDK.KKKK.",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KKKK......",
    "................",
];
const ART_AXE: [&str; 16] = [
    "................",
    "......KKKKKKK...",
    "......KdDXYYYXK.",
    "......KdDXYYXXxK",
    "......KdDXXXXxK.",
    "......KdDKKxxKK.",
    "......KdDK.KKK..",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KKKK......",
    "................",
];
const ART_SHOVEL: [&str; 16] = [
    "................",
    "......KKKK......",
    ".....KXYYXK.....",
    "....KXXYYXxK....",
    "....KXXXXXxK....",
    "....KXXXXxxK....",
    ".....KXXxxK.....",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "......KdDK......",
    "....KKKdDKKK....",
    "....KdddDDDK....",
    "....KKKKKKKK....",
];

// ---------------------------------------------------------------------
// CORACOES (9 x 8 pixels)
// ---------------------------------------------------------------------
const ART_HEART_FULL: [&str; 8] = [
    ".KKK.KKK.",
    "KMWMKMMMK",
    "KMMMMMMMK",
    "KMMMMMMMK",
    ".KMMMMMK.",
    "..KMMMK..",
    "...KMK...",
    "....K....",
];
const ART_HEART_HALF: [&str; 8] = [
    ".KKK.KKK.",
    "KMWMKkkkK",
    "KMMMMkkkK",
    "KMMMMkkkK",
    ".KMMkkkK.",
    "..KMMkK..",
    "...KMK...",
    "....K....",
];
const ART_HEART_EMPTY: [&str; 8] = [
    ".KKK.KKK.",
    "KkkkKkkkK",
    "KkkkkkkkK",
    "KkkkkkkkK",
    ".KkkkkkK.",
    "..KkkkK..",
    "...KkK...",
    "....K....",
];


// =====================================================================
// ARTE NOVA DA FASE 1.5  (blocos novos, arvores e cabelo feminino)
// =====================================================================

// Preto total: o "fundo" do interior das casas
const ART_VOID: [&str; 16] = [
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
    "KKKKKKKKKKKKKKKK",
];

// Bancada de trabalho vista de cima
const ART_WORKBENCH: [&str; 16] = [
    "FFFFFFFFFFFFFFFF",
    "FiiiiFiiiiFiiiiF",
    "FifffFffffFffffF",
    "FifkkFffffFfKffF",
    "FifffFffffFffffF",
    "FFFFFFFFFFFFFFFF",
    "FifffFffffFffffF",
    "FifffFffffFffffF",
    "FifffFfmmfFffffF",
    "FifffFffffFffffF",
    "FFFFFFFFFFFFFFFF",
    "FifffFffffFffffF",
    "FifKfFffffFfkkfF",
    "FifffFffffFffffF",
    "FifffFffffFffffF",
    "FFFFFFFFFFFFFFFF",
];

// Porta fechada (a aberta aparece quando voce chega perto)
const ART_DOOR: [&str; 16] = [
    "FFFFFFFFFFFFFFFF",
    "FfiffFfiffFfiffF",
    "FfiffFfiffFfiffF",
    "FffffFffffFffffF",
    "FfiffFfiffFfiffF",
    "FfiffFfiffFfiffF",
    "FffffFffffFffffF",
    "FfiffFfiffFfiffF",
    "FfiffFfiffFKmffF",
    "FffffFffffFKmffF",
    "FfiffFfiffFfiffF",
    "FfiffFfiffFfiffF",
    "FffffFffffFffffF",
    "FfiffFfiffFfiffF",
    "FfiffFfiffFfiffF",
    "FFFFFFFFFFFFFFFF",
];

const ART_DOOR_OPEN: [&str; 16] = [
    "FFFFFFFFFFFFFFFF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFJJJJJJJJJJF",
    "FfffFjjjjjjjjjjF",
    "FfffFjjjjjjjjjjF",
    "FfffFjjjjjjjjjjF",
    "FFFFFFFFFFFFFFFF",
];

// Parede do interior da casa (nao quebra)
const ART_HOUSEWALL: [&str; 16] = [
    "DDDDDDDDDDDDDDDD",
    "dddddddDdddddddd",
    "ddtddddDddddtddd",
    "dddddddDddtddddd",
    "dddddddDdddddddd",
    "DDDDDDDDDDDDDDDD",
    "dddDdddddddDdddd",
    "dtdDddtddddDdddd",
    "dddDdddddddDdtdd",
    "dddDdddddddDdddd",
    "DDDDDDDDDDDDDDDD",
    "dddddddDdddddddd",
    "dddtdddDddddddtd",
    "dddddddDdddtdddd",
    "dddddddDdddddddd",
    "DDDDDDDDDDDDDDDD",
];

// Muda de arvore
const ART_SAPLING: [&str; 16] = [
    "................",
    "................",
    "................",
    "................",
    "................",
    "......yy.yy.....",
    ".....ygggggy....",
    ".....gGgGggG....",
    "......gGgGg.....",
    ".......GgG......",
    "........d.......",
    "........d.......",
    "........D.......",
    ".......dDd......",
    "................",
    "................",
];

const ART_STICK: [&str; 16] = [
    "................",
    ".............KK.",
    "............KtdK",
    "...........KtdK.",
    "..........KtdK..",
    ".........KtdK...",
    "........KtdK....",
    ".......KtdK.....",
    "......KtdK......",
    ".....KtdK.......",
    "....KtdK........",
    "...KtdK.........",
    "..KtdK..........",
    "...KK...........",
    "................",
    "................",
];

// ---------------------------------------------------------------------
// ARVORES (sprites grandes, nao sao mais blocos de folha em volta do tronco)
//   pequena 16x24 | media 24x32 | grande 32x40
//   Letras da copa: L(contorno) G(sombra) g(meio) y(luz)  | tronco: D d t
//   Na neve a copa e recolorida automaticamente (veja build_tree_tex).
// ---------------------------------------------------------------------
const ART_TREE_S: [&str; 24] = [
    "................",
    "......LLLLL.....",
    "....LLyyygyLL...",
    "...LyyyyygyyL...",
    "..LyyyyygggggL..",
    "..LyyygggggggL..",
    ".LyyyggggggggGL.",
    ".LyyygggggGgGGL.",
    ".LyygggGggGGGGL.",
    ".LgygggggGGGGLL.",
    ".LyggggGGGGGGL..",
    "..LggGgLGLGGGL..",
    "...LGGLGLGGLGL..",
    "....LLGGGGLLL...",
    "......LLLL......",
    "......DddD......",
    "......DtdD......",
    "......DddD......",
    "......DtdD......",
    "......DddD......",
    "......DtdD......",
    "......DddD......",
    ".....DtdddD.....",
    ".....DddddD.....",
];

const ART_TREE_M: [&str; 32] = [
    "...........L............",
    ".........LLyLLLLL.......",
    "......LLLyyyyyyyyL......",
    ".....LyyyyyygyygygL.....",
    "....LyyyyyyyygyyyygL....",
    "...LyyyyyyyyyyygygggL...",
    "..LyyyyyyyygygygyggggL..",
    "..LyyyyyygyyygggggGGgL..",
    "..LyyyyygggggyGGgGgLgLL.",
    ".LyyyggggygyggggGLGGGGL.",
    ".LyyygyggggggggggGGGGGL.",
    ".LggggygggggggGGgGGGGGL.",
    ".LgyyggygGGggGGGGGGGGGL.",
    "..LgggggggGggGGGGGGGGGL.",
    "..LgggggGLggGGGGGGGGGL..",
    "..LgggggGGgGGGGGGGGGGL..",
    "...LGggggGGgLGGGGGGGL...",
    "....LgLGGGGLGGGGGGGLL...",
    ".....LGgGGGGGGGGGGL.....",
    "......LLLGGGGGLLLL......",
    ".........LLLLLL.........",
    ".........DddddD.........",
    ".........DtdddD.........",
    ".........DddddD.........",
    ".........DtdddD.........",
    ".........DddddD.........",
    ".........DtdddD.........",
    ".........DddddD.........",
    ".........DtdddD.........",
    ".........DddddD.........",
    "........DtdddddD........",
    "........DddddddD........",
];

const ART_TREE_L: [&str; 40] = [
    "................................",
    "............LLLLLLLL............",
    ".........LLLyyyyyyyyLLL.........",
    ".......LLyyyyyyyyyyygyyLL.......",
    ".....LLyyyyyyyyygggyyygggL......",
    "....LyyyyyyyyyyyyyyygyggggL.....",
    "....LyyyyyyyygygygyygygggggL....",
    "...LyyyyyyyyyyyyyyyyggggggggL...",
    "...LyyyyyygyyygyygggygggggggL...",
    "..LyyyyyyyygygggyggggggGgggGGL..",
    "..LyyyyyyyyggygggggggGgggggGgL..",
    ".LyyyyygygygyyyyygGgggggggGgGGL.",
    ".LyyyyyyygyygggggggLGGgGGGGGGGL.",
    ".LgyyyygyyggggggggggGGgGgGGGGGL.",
    ".LgyggygggyggggggggggGGGgGGGGGL.",
    ".LyyggggyggggggggggLggGGGGGGGGL.",
    ".LggggggggggggggggGggGgGGGGGGGL.",
    "..LgyggggggggggGGGgGGLGGGGGGGL..",
    "..LggggggggGgGgGGGGGLGGGGGGGGL..",
    "...LgggGLggGGggGGGGLGGGGGGGGGL..",
    "...LggggGgGggGLGGGGGGGGGGLLGL...",
    "....LgggGGgGgGGGGGGGGLGLGLGL....",
    ".....LGLGgGGGGGGGGGLGLGGGGL.....",
    ".....LLGGGgGGGGGGGGGGGGLGL......",
    ".......LLLGGGGGGLLGLGLGLL.......",
    ".........LLLGGGGLLGGLLL.........",
    "............LLLLLLLL............",
    "............DddddddD............",
    "............DtdddddD............",
    "............DddddddD............",
    "............DtdddddD............",
    "............DddddddD............",
    "............DtdddddD............",
    "............DddddddD............",
    "............DtdddddD............",
    "............DddddddD............",
    "............DtdddddD............",
    "............DddddddD............",
    "...........DtdddddddD...........",
    "...........DddddddddD...........",
];

// ---------------------------------------------------------------------
// CABELO FEMININO: desenhado POR CIMA do corpo (so os pixels que nao sao '.').
// Cada vista tem 7 linhas na cabeca e 4 linhas no tronco. Usa as letras H e h
// (cor do cabelo definida na skin).
// ---------------------------------------------------------------------
struct HairArt {
    head: [&'static str; 7],
    torso: [&'static str; 4],
}

/// Mesma ordem das vistas do jogador: 0=S 1=SE 2=E 3=NE 4=N
fn female_hair() -> [HairArt; 5] {
    [
        HairArt {
            head: [
                "............",
                "............",
                "............",
                "...H....H...",
                "..H......H..",
                "..H......H..",
                "............",
            ],
            torso: [
                "..HH....HH..",
                "..HH....HH..",
                "..H......H..",
                "............",
            ],
        },
        HairArt {
            head: [
                "............",
                "............",
                "............",
                "............",
                "...H.....H..",
                "...H.....H..",
                "............",
            ],
            torso: [
                "..HH....HH..",
                "..HH.....H..",
                "..H.........",
                "............",
            ],
        },
        HairArt {
            head: [
                "............",
                "............",
                "............",
                "............",
                "....H.......",
                "....H.......",
                "............",
            ],
            torso: [
                "..HHH.......",
                "..HHH.......",
                "..HH........",
                "............",
            ],
        },
        HairArt {
            head: [
                "............",
                "............",
                "............",
                "............",
                "............",
                "............",
                "............",
            ],
            torso: [
                "..HHHHHHHH..",
                "..HHHHHHHH..",
                "...HHHHHH...",
                "............",
            ],
        },
        HairArt {
            head: [
                "............",
                "............",
                "............",
                "............",
                "............",
                "............",
                "............",
            ],
            torso: [
                "..HHHHHHHH..",
                "..HHHHHHHH..",
                "...HHHHHH...",
                "............",
            ],
        },
    ]
}


// =====================================================================
// SECAO 2 - CONFIGURACOES (constantes)
//  (as configuracoes que o jogador muda no menu estao na struct Settings)
// =====================================================================
const TILE: usize = 16; // pixels de cada bloco na arte
const CHUNK: i32 = 16; // blocos por lado de cada chunk
const HUMAN_W: usize = 12;
const HUMAN_H: usize = 16;

// Atlas: [0..24) blocos | [24..36) ferramentas | 36 sementes | 37 graveto | 38..41 coracoes
const ATLAS_COLS: usize = 8;
const ATLAS_ROWS: usize = 6;
const T_TOOLS: usize = 24;
const T_SEEDS: usize = 36;
const T_STICK: usize = 37;
const T_HEART_FULL: usize = 38;
const T_HEART_HALF: usize = 39;
const T_HEART_EMPTY: usize = 40;

const PLAYER_SPEED: f32 = 4.3; // blocos por segundo
const PLAYER_HALF: f32 = 0.26; // meia largura da caixa de colisao
const REACH: f32 = 1.25; // raio da orbita do cursor (alcance)
const MAX_HP: i32 = 20;
const STARTER_KIT: bool = false; // true = comeca com ferramentas e blocos (teste)
const FIXED_SEED: Option<u32> = None; // ex.: Some(1234) para sempre o mesmo mundo

const SAPLING_GROW_SECS: f32 = 75.0; // tempo da muda virar arvore
const TREE_WARN: f32 = 0.7; // a arvore balanca antes de cair (segundos)
const TREE_FALL: f32 = 0.9; // duracao da queda
const TREE_LINGER: f32 = 1.6; // quanto tempo o tronco caido fica no chao
const ROTATION_SIGN: f32 = 1.0; // se a arvore cair visualmente para o lado errado, use -1.0
const HOUSE_MAX: i32 = 12; // maior vao (em blocos) que ainda conta como casa

const CONFIG_FILE: &str = "bio_config.txt";
const SKIN_DIR: &str = "skins";

const UI_SLOT: f32 = 56.0;
const UI_GAP: f32 = 4.0;
const UI_PAD: f32 = 16.0;

// =====================================================================
// SECAO 3 - BLOCOS E ITENS
// =====================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ToolKind {
    Pickaxe,
    Axe,
    Shovel,
}

impl ToolKind {
    fn name(self) -> &'static str {
        match self {
            ToolKind::Pickaxe => "Picareta",
            ToolKind::Axe => "Machado",
            ToolKind::Shovel => "Pa",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tier {
    Wood,
    Stone,
    Iron,
    Diamond,
}

impl Tier {
    fn from_index(i: usize) -> Tier {
        match i {
            0 => Tier::Wood,
            1 => Tier::Stone,
            2 => Tier::Iron,
            _ => Tier::Diamond,
        }
    }
    fn speed(self) -> f32 {
        match self {
            Tier::Wood => 2.0,
            Tier::Stone => 4.0,
            Tier::Iron => 6.0,
            Tier::Diamond => 8.0,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Tier::Wood => "madeira",
            Tier::Stone => "pedra",
            Tier::Iron => "ferro",
            Tier::Diamond => "diamante",
        }
    }
    /// (principal, sombra, brilho) usados nos 'X' 'x' 'Y' das ferramentas
    fn colors(self) -> (Color, Color, Color) {
        match self {
            Tier::Wood => (hex(0xb98a50), hex(0x94683a), hex(0xd1a468)),
            Tier::Stone => (hex(0x8a8a92), hex(0x6a6a72), hex(0xa6a6ae)),
            Tier::Iron => (hex(0xd8d8e0), hex(0xa8a8b4), hex(0xf4f4f8)),
            Tier::Diamond => (hex(0x4fe3d8), hex(0x2bb0a8), hex(0xa8fff4)),
        }
    }
}

/// Cada celula do mundo tem 3 camadas:
///   1) CHAO   (grama, terra, agua, buraco...)
///   2) PISO   (tabuas, pedregulho... colocados POR CIMA do chao, da para andar)
///   3) BLOCO  (paredes, arvores, bancada, porta, decoracao...)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Block {
    Air,
    // --- chao
    Grass,
    Dirt,
    Sand,
    Snow,
    Water,
    Hole,
    Void, // vazio preto (fora das paredes do interior de uma casa)
    // --- blocos
    Stone,
    Cobble,
    Planks,
    Log,
    Leaves,
    Cactus,
    // --- decoracao (nao bloqueia)
    TallGrass,
    Flower,
    // --- casa e crafting
    Workbench,
    Door,
    DoorOpen, // so existe como visual (porta aberta quando voce chega perto)
    HouseWall,
    Sapling,
    // --- arvores (desenhadas como sprites grandes)
    TreeS,
    TreeM,
    TreeL,
}

const BLOCK_COUNT: usize = 24;
const ALL_BLOCKS: [Block; BLOCK_COUNT] = [
    Block::Air,
    Block::Grass,
    Block::Dirt,
    Block::Sand,
    Block::Snow,
    Block::Water,
    Block::Hole,
    Block::Void,
    Block::Stone,
    Block::Cobble,
    Block::Planks,
    Block::Log,
    Block::Leaves,
    Block::Cactus,
    Block::TallGrass,
    Block::Flower,
    Block::Workbench,
    Block::Door,
    Block::DoorOpen,
    Block::HouseWall,
    Block::Sapling,
    Block::TreeS,
    Block::TreeM,
    Block::TreeL,
];

fn block_art(b: Block) -> &'static [&'static str] {
    match b {
        Block::Air | Block::TreeS | Block::TreeM | Block::TreeL => &[],
        Block::Grass => &ART_GRASS,
        Block::Dirt => &ART_DIRT,
        Block::Sand => &ART_SAND,
        Block::Snow => &ART_SNOW,
        Block::Water => &ART_WATER,
        Block::Hole => &ART_HOLE,
        Block::Void => &ART_VOID,
        Block::Stone => &ART_STONE,
        Block::Cobble => &ART_COBBLE,
        Block::Planks => &ART_PLANKS,
        Block::Log => &ART_LOG,
        Block::Leaves => &ART_LEAVES,
        Block::Cactus => &ART_CACTUS,
        Block::TallGrass => &ART_TALLGRASS,
        Block::Flower => &ART_FLOWER,
        Block::Workbench => &ART_WORKBENCH,
        Block::Door => &ART_DOOR,
        Block::DoorOpen => &ART_DOOR_OPEN,
        Block::HouseWall => &ART_HOUSEWALL,
        Block::Sapling => &ART_SAPLING,
    }
}

impl Block {
    fn solid(self) -> bool {
        matches!(
            self,
            Block::Stone
                | Block::Cobble
                | Block::Planks
                | Block::Log
                | Block::Leaves
                | Block::Cactus
                | Block::Workbench
                | Block::HouseWall
                | Block::TreeS
                | Block::TreeM
                | Block::TreeL
        )
    }
    fn is_tree(self) -> bool {
        matches!(self, Block::TreeS | Block::TreeM | Block::TreeL)
    }
    /// 0 = pequena, 1 = media, 2 = grande
    fn tree_size(self) -> usize {
        match self {
            Block::TreeS => 0,
            Block::TreeM => 1,
            _ => 2,
        }
    }
    fn is_deco(self) -> bool {
        matches!(self, Block::TallGrass | Block::Flower | Block::Sapling)
    }
    /// Blocos "em pe": desenhados com topo + face da frente (efeito 3D)
    fn raised(self) -> bool {
        matches!(
            self,
            Block::Stone
                | Block::Cobble
                | Block::Planks
                | Block::Log
                | Block::Leaves
                | Block::Workbench
                | Block::HouseWall
                | Block::Door
        )
    }
    fn floor_walkable(self) -> bool {
        !matches!(self, Block::Water | Block::Hole | Block::Void)
    }
    /// tiles que preenchem os 16x16 inteiros (os outros tem partes transparentes)
    fn opaque_tile(self) -> bool {
        !matches!(
            self,
            Block::Air
                | Block::TallGrass
                | Block::Flower
                | Block::Cactus
                | Block::Sapling
                | Block::TreeS
                | Block::TreeM
                | Block::TreeL
        )
    }
    /// Dureza base em "segundos". None = nao quebra.
    fn hardness(self) -> Option<f32> {
        Some(match self {
            Block::Grass => 0.6,
            Block::Dirt => 0.5,
            Block::Sand => 0.5,
            Block::Snow => 0.2,
            Block::Stone => 1.5,
            Block::Cobble => 2.0,
            Block::Planks => 2.0,
            Block::Log => 2.0,
            Block::Leaves => 0.2,
            Block::Cactus => 0.4,
            Block::TallGrass => 0.0,
            Block::Flower => 0.0,
            Block::Sapling => 0.0,
            Block::Workbench => 2.0,
            Block::Door => 1.0,
            Block::TreeS => 2.0,
            Block::TreeM => 3.0,
            Block::TreeL => 4.5,
            Block::Air
            | Block::Water
            | Block::Hole
            | Block::Void
            | Block::DoorOpen
            | Block::HouseWall => return None,
        })
    }
    /// Ferramenta que acelera a quebra deste bloco.
    fn tool_pref(self) -> Option<ToolKind> {
        match self {
            Block::Grass | Block::Dirt | Block::Sand | Block::Snow => Some(ToolKind::Shovel),
            Block::Stone | Block::Cobble => Some(ToolKind::Pickaxe),
            Block::Planks
            | Block::Log
            | Block::Cactus
            | Block::Workbench
            | Block::Door
            | Block::TreeS
            | Block::TreeM
            | Block::TreeL => Some(ToolKind::Axe),
            _ => None,
        }
    }
    /// Se retornar Some(tier): so solta item com a ferramenta certa desse nivel ou melhor.
    fn needs_tool(self) -> Option<Tier> {
        match self {
            Block::Stone | Block::Cobble => Some(Tier::Wood),
            _ => None,
        }
    }
    /// Cor representativa (particulas e preenchimento do tile)
    fn color(self) -> Color {
        hex(match self {
            Block::Grass => 0x5fae3b,
            Block::Dirt => 0x8b5a2b,
            Block::Sand => 0xe8d68e,
            Block::Snow => 0xf1f6fa,
            Block::Water => 0x3b7dd8,
            Block::Hole => 0x1c140f,
            Block::Void => 0x000000,
            Block::Stone => 0x8a8a92,
            Block::Cobble => 0x7a7a82,
            Block::Planks => 0xb98a50,
            Block::Log => 0x6a4122,
            Block::Leaves => 0x3f8a2f,
            Block::Cactus => 0x3f9a47,
            Block::TallGrass => 0x5fae3b,
            Block::Flower => 0xe0405a,
            Block::Workbench => 0xb98a50,
            Block::Door => 0xb98a50,
            Block::DoorOpen => 0x2c2119,
            Block::HouseWall => 0x6a4122,
            Block::Sapling => 0x5fae3b,
            Block::TreeS | Block::TreeM | Block::TreeL => 0x3f8a2f,
            Block::Air => 0xffffff,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Item {
    Dirt,
    Sand,
    Snow,
    Cobble,
    Planks,
    Log,
    Leaves,
    Cactus,
    Flower,
    Seeds, // usadas no trigo (Fase 4)
    Stick,
    Workbench,
    Door,
    Sapling,
    Tool(ToolKind, Tier),
}

enum Placement {
    Floor(Block), // troca o chao
    Solid(Block), // bloco (ou piso, se der)
    Deco(Block),  // decoracao
}

impl Item {
    fn name(self) -> String {
        match self {
            Item::Dirt => "Terra".to_string(),
            Item::Sand => "Areia".to_string(),
            Item::Snow => "Neve".to_string(),
            Item::Cobble => "Pedregulho".to_string(),
            Item::Planks => "Tabuas".to_string(),
            Item::Log => "Tronco".to_string(),
            Item::Leaves => "Bloco de folhas".to_string(),
            Item::Cactus => "Cacto".to_string(),
            Item::Flower => "Flor".to_string(),
            Item::Seeds => "Sementes".to_string(),
            Item::Stick => "Graveto".to_string(),
            Item::Workbench => "Bancada".to_string(),
            Item::Door => "Porta".to_string(),
            Item::Sapling => "Muda de arvore".to_string(),
            Item::Tool(k, t) => format!("{} de {}", k.name(), t.name()),
        }
    }
    fn max_stack(self) -> u8 {
        match self {
            Item::Tool(..) => 1,
            _ => 64,
        }
    }
    fn icon_tile(self) -> usize {
        match self {
            Item::Dirt => Block::Dirt as usize,
            Item::Sand => Block::Sand as usize,
            Item::Snow => Block::Snow as usize,
            Item::Cobble => Block::Cobble as usize,
            Item::Planks => Block::Planks as usize,
            Item::Log => Block::Log as usize,
            Item::Leaves => Block::Leaves as usize,
            Item::Cactus => Block::Cactus as usize,
            Item::Flower => Block::Flower as usize,
            Item::Seeds => T_SEEDS,
            Item::Stick => T_STICK,
            Item::Workbench => Block::Workbench as usize,
            Item::Door => Block::Door as usize,
            Item::Sapling => Block::Sapling as usize,
            Item::Tool(k, t) => T_TOOLS + (k as usize) * 4 + (t as usize),
        }
    }
    fn placement(self) -> Option<Placement> {
        match self {
            Item::Dirt => Some(Placement::Floor(Block::Dirt)),
            Item::Sand => Some(Placement::Floor(Block::Sand)),
            Item::Snow => Some(Placement::Floor(Block::Snow)),
            Item::Cobble => Some(Placement::Solid(Block::Cobble)),
            Item::Planks => Some(Placement::Solid(Block::Planks)),
            Item::Log => Some(Placement::Solid(Block::Log)),
            Item::Leaves => Some(Placement::Solid(Block::Leaves)),
            Item::Cactus => Some(Placement::Solid(Block::Cactus)),
            Item::Workbench => Some(Placement::Solid(Block::Workbench)),
            Item::Door => Some(Placement::Solid(Block::Door)),
            Item::Flower => Some(Placement::Deco(Block::Flower)),
            Item::Sapling => Some(Placement::Deco(Block::Sapling)),
            Item::Seeds | Item::Stick | Item::Tool(..) => None,
        }
    }
    /// Se este item pode virar PISO (camada acima do chao), qual bloco desenha.
    fn cover_block(self) -> Option<Block> {
        match self {
            Item::Dirt => Some(Block::Dirt),
            Item::Sand => Some(Block::Sand),
            Item::Snow => Some(Block::Snow),
            Item::Cobble => Some(Block::Cobble),
            Item::Planks => Some(Block::Planks),
            Item::Log => Some(Block::Log),
            Item::Leaves => Some(Block::Leaves),
            _ => None,
        }
    }
}

fn tool_of(held: Option<Item>) -> Option<(ToolKind, Tier)> {
    match held {
        Some(Item::Tool(k, t)) => Some((k, t)),
        _ => None,
    }
}

/// O bloco solta item com a ferramenta que o jogador esta segurando?
fn can_harvest(b: Block, held: Option<Item>) -> bool {
    match b.needs_tool() {
        None => true,
        Some(min) => match (b.tool_pref(), tool_of(held)) {
            (Some(pk), Some((k, t))) => k == pk && t >= min,
            _ => false,
        },
    }
}

/// Tempo (s) para quebrar. Sem a ferramenta certa fica bem mais lento.
fn break_time(b: Block, held: Option<Item>) -> Option<f32> {
    let h = b.hardness()?;
    let speed = match (b.tool_pref(), tool_of(held)) {
        (Some(pk), Some((k, t))) if k == pk => t.speed(),
        _ => 1.0,
    };
    Some(if can_harvest(b, held) { h * 1.5 / speed } else { h * 5.0 })
}

/// Item que um bloco/piso solta (arvores sao tratadas a parte).
fn drop_for(b: Block, held: Option<Item>) -> Option<(Item, u8)> {
    if !can_harvest(b, held) {
        return None;
    }
    let item = match b {
        Block::Grass | Block::Dirt => Item::Dirt,
        Block::Sand => Item::Sand,
        Block::Snow => Item::Snow,
        Block::Stone | Block::Cobble => Item::Cobble,
        Block::Planks => Item::Planks,
        Block::Log => Item::Log,
        Block::Leaves => Item::Leaves,
        Block::Cactus => Item::Cactus,
        Block::TallGrass => Item::Seeds,
        Block::Flower => Item::Flower,
        Block::Workbench => Item::Workbench,
        Block::Door => Item::Door,
        Block::Sapling => Item::Sapling,
        _ => return None,
    };
    Some((item, 1))
}

// ---------------------------------------------------------------------
// RECEITAS DE CRAFTING (estilo Minecraft)
//   shaped(): formato fixo ('M' = material, ' ' = vazio). Pode ser espelhada.
//   shapeless(): ingredientes em qualquer posicao.
//   Adicione novas receitas em build_recipes().
// ---------------------------------------------------------------------
struct Recipe {
    shapeless: bool,
    w: usize,
    h: usize,
    cells: Vec<Option<Item>>,
    out: (Item, u8),
}

fn shaped(rows: &[&str], legend: &[(char, Item)], out: Item, n: u8) -> Recipe {
    let h = rows.len();
    let w = rows[0].chars().count();
    let mut cells = Vec::new();
    for r in rows {
        for ch in r.chars() {
            cells.push(legend.iter().find(|(c, _)| *c == ch).map(|(_, it)| *it));
        }
    }
    Recipe { shapeless: false, w, h, cells, out: (out, n) }
}

fn shapeless(items: &[Item], out: Item, n: u8) -> Recipe {
    Recipe {
        shapeless: true,
        w: 0,
        h: 0,
        cells: items.iter().map(|i| Some(*i)).collect(),
        out: (out, n),
    }
}

fn build_recipes() -> Vec<Recipe> {
    let mut v = Vec::new();
    v.push(shapeless(&[Item::Log], Item::Planks, 4));
    v.push(shaped(&["P", "P"], &[('P', Item::Planks)], Item::Stick, 4));
    v.push(shaped(&["PP", "PP"], &[('P', Item::Planks)], Item::Workbench, 1));
    v.push(shaped(&["PP", "PP", "PP"], &[('P', Item::Planks)], Item::Door, 3));
    for (mat, tier) in [(Item::Planks, Tier::Wood), (Item::Cobble, Tier::Stone)] {
        let leg = [('M', mat), ('S', Item::Stick)];
        v.push(shaped(&["MMM", " S ", " S "], &leg, Item::Tool(ToolKind::Pickaxe, tier), 1));
        v.push(shaped(&["MM", "MS", " S"], &leg, Item::Tool(ToolKind::Axe, tier), 1));
        v.push(shaped(&["M", "S", "S"], &leg, Item::Tool(ToolKind::Shovel, tier), 1));
    }
    v
}

/// Procura uma receita para a grade (n x n, guardada em ordem de linhas).
fn match_recipe(recipes: &[Recipe], grid: &[Option<ItemStack>; 9], n: usize) -> Option<(Item, u8)> {
    let cells: Vec<Option<Item>> = (0..n * n).map(|i| grid[i].map(|s| s.item)).collect();
    let (mut minx, mut miny, mut maxx, mut maxy) = (n, n, 0usize, 0usize);
    let mut any = false;
    for y in 0..n {
        for x in 0..n {
            if cells[y * n + x].is_some() {
                any = true;
                minx = minx.min(x);
                miny = miny.min(y);
                maxx = maxx.max(x);
                maxy = maxy.max(y);
            }
        }
    }
    if !any {
        return None;
    }
    let gw = maxx - minx + 1;
    let gh = maxy - miny + 1;
    let mut sub: Vec<Option<Item>> = Vec::new();
    for y in miny..=maxy {
        for x in minx..=maxx {
            sub.push(cells[y * n + x]);
        }
    }
    let mut mirrored: Vec<Option<Item>> = Vec::new();
    for y in 0..gh {
        for x in 0..gw {
            mirrored.push(sub[y * gw + (gw - 1 - x)]);
        }
    }
    let have: Vec<Item> = sub.iter().flatten().copied().collect();

    for r in recipes {
        if r.shapeless {
            let mut pool = have.clone();
            let mut ok = pool.len() == r.cells.len();
            if ok {
                for it in r.cells.iter().flatten() {
                    if let Some(p) = pool.iter().position(|x| x == it) {
                        pool.remove(p);
                    } else {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                return Some(r.out);
            }
        } else if r.w == gw && r.h == gh && (sub == r.cells || mirrored == r.cells) {
            return Some(r.out);
        }
    }
    None
}

// =====================================================================
// SECAO 4 - SPRITES NA MEMORIA (atlas de blocos, jogador com skin, arvores)
// =====================================================================

/// Pinta um desenho em texto dentro de uma imagem. Linhas/colunas a mais sao ignoradas.
fn paint(
    img: &mut Image,
    ox: usize,
    oy: usize,
    rows: &[&str],
    max_w: usize,
    max_h: usize,
    map: &dyn Fn(char) -> Option<Color>,
) {
    for (y, row) in rows.iter().enumerate() {
        if y >= max_h {
            break;
        }
        for (x, ch) in row.chars().enumerate() {
            if x >= max_w {
                break;
            }
            if let Some(col) = map(ch) {
                img.set_pixel((ox + x) as u32, (oy + y) as u32, col);
            }
        }
    }
}

fn tile_origin(idx: usize) -> (usize, usize) {
    ((idx % ATLAS_COLS) * TILE, (idx / ATLAS_COLS) * TILE)
}

fn fill_tile(img: &mut Image, idx: usize, color: Color) {
    let (ox, oy) = tile_origin(idx);
    for y in 0..TILE {
        for x in 0..TILE {
            img.set_pixel((ox + x) as u32, (oy + y) as u32, color);
        }
    }
}

fn put_tile(img: &mut Image, idx: usize, rows: &[&str], map: &dyn Fn(char) -> Option<Color>) {
    let (ox, oy) = tile_origin(idx);
    paint(img, ox, oy, rows, TILE, TILE, map);
}

fn build_atlas() -> Texture2D {
    let mut img = Image::gen_image_color((ATLAS_COLS * TILE) as u16, (ATLAS_ROWS * TILE) as u16, BLANK);

    // blocos
    for b in ALL_BLOCKS.iter() {
        let idx = *b as usize;
        // tiles opacos recebem uma cor de fundo (protege de buracos se alguma linha da arte ficar curta)
        if b.opaque_tile() {
            fill_tile(&mut img, idx, b.color());
        }
        put_tile(&mut img, idx, block_art(*b), &palette);
    }

    // ferramentas: 3 desenhos x 4 materiais
    let tool_arts: [&[&str]; 3] = [&ART_PICKAXE, &ART_AXE, &ART_SHOVEL];
    for (k, art) in tool_arts.iter().enumerate() {
        for t in 0..4usize {
            let (c1, c2, c3) = Tier::from_index(t).colors();
            let map = move |ch: char| -> Option<Color> {
                match ch {
                    'X' => Some(c1),
                    'x' => Some(c2),
                    'Y' => Some(c3),
                    _ => palette(ch),
                }
            };
            put_tile(&mut img, T_TOOLS + k * 4 + t, art, &map);
        }
    }

    // sementes, graveto e coracoes
    put_tile(&mut img, T_SEEDS, &ART_SEEDS, &palette);
    put_tile(&mut img, T_STICK, &ART_STICK, &palette);
    put_tile(&mut img, T_HEART_FULL, &ART_HEART_FULL, &palette);
    put_tile(&mut img, T_HEART_HALF, &ART_HEART_HALF, &palette);
    put_tile(&mut img, T_HEART_EMPTY, &ART_HEART_EMPTY, &palette);

    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    tex
}

// ---------------------------------------------------------------------
// SKIN DO PERSONAGEM: genero + 6 cores. As sombras/brilhos sao calculados
// automaticamente a partir de cada cor.
// ---------------------------------------------------------------------
#[derive(Clone, PartialEq, Debug)]
struct Skin {
    name: String,
    female: bool,
    hair: [u8; 3],
    skin: [u8; 3],
    shirt: [u8; 3],
    pants: [u8; 3],
    shoes: [u8; 3],
    eyes: [u8; 3],
}

impl Skin {
    fn standard() -> Skin {
        Skin {
            name: "Bio".to_string(),
            female: false,
            hair: [0x4b, 0x2e, 0x1b],
            skin: [0xf2, 0xb8, 0x90],
            shirt: [0x3a, 0xa8, 0xbd],
            pants: [0x3a, 0x4a, 0xa8],
            shoes: [0x4a, 0x4a, 0x55],
            eyes: [0x1a, 0x1a, 0x2e],
        }
    }
    /// Acesso por indice (0 cabelo, 1 pele, 2 camisa, 3 calca, 4 sapato, 5 olhos)
    fn part(&self, i: usize) -> [u8; 3] {
        match i {
            0 => self.hair,
            1 => self.skin,
            2 => self.shirt,
            3 => self.pants,
            4 => self.shoes,
            _ => self.eyes,
        }
    }
    fn set_part(&mut self, i: usize, c: [u8; 3]) {
        match i {
            0 => self.hair = c,
            1 => self.skin = c,
            2 => self.shirt = c,
            3 => self.pants = c,
            4 => self.shoes = c,
            _ => self.eyes = c,
        }
    }
}

const SKIN_PART_NAMES: [&str; 6] = ["Cabelo", "Pele", "Camisa", "Calca", "Sapato", "Olhos"];

fn mulc(c: [u8; 3], f: f32) -> Color {
    Color::new(
        (c[0] as f32 / 255.0 * f).min(1.0),
        (c[1] as f32 / 255.0 * f).min(1.0),
        (c[2] as f32 / 255.0 * f).min(1.0),
        1.0,
    )
}

/// Letra do desenho -> cor, aplicando as cores da skin.
fn skin_color(skin: &Skin, ch: char) -> Option<Color> {
    match ch {
        'H' => Some(mulc(skin.hair, 1.0)),
        'h' => Some(mulc(skin.hair, 1.4)),
        'S' => Some(mulc(skin.skin, 1.0)),
        's' => Some(mulc(skin.skin, 0.82)),
        'E' => Some(mulc(skin.eyes, 1.0)),
        'B' => Some(mulc(skin.shirt, 1.0)),
        'b' => Some(mulc(skin.shirt, 0.75)),
        'P' => Some(mulc(skin.pants, 1.0)),
        'p' => Some(mulc(skin.pants, 0.75)),
        'k' => Some(mulc(skin.shoes, 1.0)),
        _ => palette(ch),
    }
}

/// Folha do jogador: 4 colunas (parado, passo A, passo B, interagir) x 5 linhas (vistas).
fn build_human_image(skin: &Skin) -> Image {
    let views = player_views();
    let hair = female_hair();
    let map = |ch: char| skin_color(skin, ch);
    let mut img = Image::gen_image_color((HUMAN_W * 4) as u16, (HUMAN_H * 5) as u16, BLANK);
    for (v, art) in views.iter().enumerate() {
        for f in 0..4usize {
            let mut rows: Vec<&str> = Vec::with_capacity(HUMAN_H);
            rows.extend_from_slice(&art.head);
            if f == 3 {
                rows.extend_from_slice(&art.point);
            } else {
                rows.extend_from_slice(&art.torso);
            }
            let legs = match f {
                1 => &art.legs[1],
                2 => &art.legs[2],
                _ => &art.legs[0],
            };
            rows.extend_from_slice(legs);
            paint(&mut img, f * HUMAN_W, v * HUMAN_H, &rows, HUMAN_W, HUMAN_H, &map);
            if skin.female {
                paint(&mut img, f * HUMAN_W, v * HUMAN_H, &hair[v].head, HUMAN_W, 7, &map);
                paint(&mut img, f * HUMAN_W, v * HUMAN_H + 7, &hair[v].torso, HUMAN_W, 4, &map);
            }
        }
    }
    img
}

fn build_human_sheet(skin: &Skin) -> Texture2D {
    let img = build_human_image(skin);
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    tex
}

struct SpriteSheet {
    tex: Texture2D,
    fw: f32,
    fh: f32,
}

impl SpriteSheet {
    /// Desenha uma vista (linha) e um quadro (coluna) da folha.
    fn draw_raw(&self, view: usize, frame: usize, flip: bool, x: f32, y: f32, scale: f32, tint: Color) {
        draw_texture_ex(
            &self.tex,
            x,
            y,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(self.fw * scale, self.fh * scale)),
                source: Some(Rect::new(frame as f32 * self.fw, view as f32 * self.fh, self.fw, self.fh)),
                flip_x: flip,
                ..Default::default()
            },
        );
    }
    /// Desenha em uma das 8 direcoes (0=L 1=SE 2=S 3=SO 4=O 5=NO 6=N 7=NE).
    fn draw(&self, dir8: usize, frame: usize, x: f32, y: f32, scale: f32, tint: Color) {
        let (view, flip) = view_for(dir8);
        self.draw_raw(view, frame, flip, x, y, scale, tint);
    }
}

/// 8 direcoes: 0=L(E) 1=SE 2=S 3=SO 4=O 5=NO 6=N 7=NE  (y cresce para baixo)
fn dir8_from_vec(v: Vec2) -> usize {
    let a = v.y.atan2(v.x);
    ((a / (PI / 4.0)).round() as i32).rem_euclid(8) as usize
}

/// direcao (0..8) -> (vista desenhada, espelhar?)
/// vistas: 0=S 1=SE 2=E 3=NE 4=N
fn view_for(dir8: usize) -> (usize, bool) {
    match dir8 {
        0 => (2, false),
        1 => (1, false),
        2 => (0, false),
        3 => (1, true),
        4 => (2, true),
        5 => (3, true),
        6 => (4, false),
        _ => (3, false),
    }
}

// ---------------------------------------------------------------------
// ARVORES: um sprite por tamanho e por variante (normal / neve)
// ---------------------------------------------------------------------
struct TreeTex {
    tex: Texture2D,
    w: f32, // largura da arte em pixels
    h: f32,
}

fn build_tree_tex(rows: &[&str], snowy: bool) -> TreeTex {
    let w = rows.iter().map(|r| r.chars().count()).max().unwrap_or(1);
    let h = rows.len();
    let mut img = Image::gen_image_color(w as u16, h as u16, BLANK);
    let map = move |ch: char| -> Option<Color> {
        if snowy {
            match ch {
                'g' => Some(hex(0x9cc7a8)),
                'G' => Some(hex(0x6aa585)),
                'y' => Some(hex(0xeaf4f0)),
                'L' => Some(hex(0x3f7a62)),
                _ => palette(ch),
            }
        } else {
            palette(ch)
        }
    };
    paint(&mut img, 0, 0, rows, w, h, &map);
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    TreeTex { tex, w: w as f32, h: h as f32 }
}

/// indice = (neve ? 3 : 0) + tamanho
fn build_trees() -> Vec<TreeTex> {
    let arts: [&[&str]; 3] = [&ART_TREE_S, &ART_TREE_M, &ART_TREE_L];
    let mut v = Vec::new();
    for snowy in [false, true] {
        for a in arts.iter() {
            v.push(build_tree_tex(a, snowy));
        }
    }
    v
}

// ---------------------------------------------------------------------
// Helpers de desenho
// ---------------------------------------------------------------------
fn tile_rect(idx: usize) -> Rect {
    let (ox, oy) = tile_origin(idx);
    Rect::new(ox as f32, oy as f32, TILE as f32, TILE as f32)
}

fn draw_tile(atlas: &Texture2D, idx: usize, x: f32, y: f32, size: f32) {
    draw_tile_tint(atlas, idx, x, y, size, WHITE);
}

fn draw_tile_tint(atlas: &Texture2D, idx: usize, x: f32, y: f32, size: f32, tint: Color) {
    draw_texture_ex(
        atlas,
        x,
        y,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            source: Some(tile_rect(idx)),
            ..Default::default()
        },
    );
}

fn draw_heart(atlas: &Texture2D, idx: usize, x: f32, y: f32) {
    let (ox, oy) = tile_origin(idx);
    draw_texture_ex(
        atlas,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(27.0, 24.0)),
            source: Some(Rect::new(ox as f32, oy as f32, 9.0, 8.0)),
            ..Default::default()
        },
    );
}

fn text_shadow(s: &str, x: f32, y: f32, size: f32, col: Color) {
    draw_text(s, x + 2.0, y + 2.0, size, Color::new(0.0, 0.0, 0.0, 0.7 * col.a));
    draw_text(s, x, y, size, col);
}

fn text_centered(s: &str, cx: f32, y: f32, size: f32, col: Color) {
    let d = measure_text(s, None, size as u16, 1.0);
    text_shadow(s, cx - d.width / 2.0, y, size, col);
}

// =====================================================================
// SECAO 5 - GERACAO DO MUNDO (ruido deterministico, sem arquivos)
// =====================================================================

fn hash_u32(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = seed ^ (x as u32).wrapping_mul(0x27d4eb2d) ^ (y as u32).wrapping_mul(0x165667b1);
    h = (h ^ (h >> 15)).wrapping_mul(0x85ebca6b);
    h = (h ^ (h >> 13)).wrapping_mul(0xc2b2ae35);
    h ^ (h >> 16)
}

fn hash01(x: i32, y: i32, seed: u32) -> f32 {
    (hash_u32(x, y, seed) >> 8) as f32 / 16777216.0
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor();
    let y0 = y.floor();
    let fx = smooth(x - x0);
    let fy = smooth(y - y0);
    let ix = x0 as i32;
    let iy = y0 as i32;
    let a = hash01(ix, iy, seed);
    let b = hash01(ix + 1, iy, seed);
    let c = hash01(ix, iy + 1, seed);
    let d = hash01(ix + 1, iy + 1, seed);
    let top = a + (b - a) * fx;
    let bot = c + (d - c) * fx;
    top + (bot - top) * fy
}

fn fbm(x: f32, y: f32, seed: u32, octaves: u32) -> f32 {
    let mut sum = 0.0;
    let mut amp = 1.0;
    let mut freq = 1.0;
    let mut norm = 0.0;
    for o in 0..octaves {
        sum += amp * value_noise(x * freq, y * freq, seed.wrapping_add(o * 101));
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum / norm
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Biome {
    Plains,
    Forest,
    Snow,
    Desert,
}

impl Biome {
    fn name(self) -> &'static str {
        match self {
            Biome::Plains => "planicie",
            Biome::Forest => "floresta",
            Biome::Snow => "neve",
            Biome::Desert => "deserto",
        }
    }
}

fn biome_at(x: i32, y: i32, seed: u32) -> Biome {
    let t = fbm(x as f32 / 110.0, y as f32 / 110.0, seed.wrapping_add(11), 3);
    let m = fbm(x as f32 / 80.0, y as f32 / 80.0, seed.wrapping_add(23), 3);
    if t < 0.40 {
        Biome::Snow
    } else if t > 0.63 {
        Biome::Desert
    } else if m > 0.52 {
        Biome::Forest
    } else {
        Biome::Plains
    }
}

fn elevation_at(x: i32, y: i32, seed: u32) -> f32 {
    fbm(x as f32 / 36.0, y as f32 / 36.0, seed.wrapping_add(37), 3)
}

/// Chao padrao do interior de uma casa, conforme o bioma onde ela foi construida.
/// (Troque aqui se quiser grama, areia etc.)
fn house_floor(b: Biome) -> Block {
    match b {
        Biome::Snow => Block::Snow,
        _ => Block::Dirt,
    }
}

/// O centro (tronco) de uma arvore? Nunca dois troncos a menos de 3 blocos.
fn is_tree_center(x: i32, y: i32, seed: u32) -> bool {
    let h = hash01(x, y, seed.wrapping_add(7));
    if h >= 0.05 {
        return false;
    }
    for dy in -2..=2 {
        for dx in -2..=2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if hash01(x + dx, y + dy, seed.wrapping_add(7)) < h {
                return false;
            }
        }
    }
    let biome = biome_at(x, y, seed);
    let e = elevation_at(x, y, seed);
    let p = match biome {
        Biome::Forest => 0.05,
        Biome::Snow => 0.02,
        Biome::Plains => 0.006,
        Biome::Desert => 0.0,
    };
    h < p && e > 0.39 && e < 0.62
}

/// Tamanho da arvore nesta posicao (muda sempre igual para o mesmo lugar).
fn tree_block_at(x: i32, y: i32, seed: u32) -> Block {
    let r = hash01(x, y, seed.wrapping_add(17));
    if r < 0.50 {
        Block::TreeS
    } else if r < 0.85 {
        Block::TreeM
    } else {
        Block::TreeL
    }
}

/// Gera uma celula do mundo: (chao, bloco). Funcao pura: mesma entrada = mesma saida.
fn gen_cell(x: i32, y: i32, seed: u32) -> (Block, Block) {
    let biome = biome_at(x, y, seed);
    let e = elevation_at(x, y, seed);
    let mut floor = match biome {
        Biome::Snow => Block::Snow,
        Biome::Desert => Block::Sand,
        _ => Block::Grass,
    };
    let water_level = if biome == Biome::Desert { 0.30 } else { 0.36 };
    if e < water_level {
        return (Block::Water, Block::Air);
    }
    if e < water_level + 0.03 && biome != Biome::Desert {
        floor = Block::Sand; // praia
    }
    if e > 0.67 && biome != Biome::Desert {
        return (Block::Dirt, Block::Stone); // afloramento de pedra
    }

    // arvores (sprites grandes de 3 tamanhos)
    if (floor == Block::Grass || floor == Block::Snow) && is_tree_center(x, y, seed) {
        return (floor, tree_block_at(x, y, seed));
    }

    // decoracao
    let r = hash01(x, y, seed.wrapping_add(13));
    match floor {
        Block::Grass => {
            let p = if biome == Biome::Forest { 0.05 } else { 0.11 };
            if r < p {
                return (floor, Block::TallGrass);
            }
            if r > 0.992 {
                return (floor, Block::Flower);
            }
        }
        Block::Sand if biome == Biome::Desert => {
            if r < 0.012 {
                return (floor, Block::Cactus);
            }
        }
        _ => {}
    }
    (floor, Block::Air)
}

fn find_spawn(seed: u32) -> Vec2 {
    for r in 0..120i32 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let mut ok = true;
                'chk: for oy in -1..=1 {
                    for ox in -1..=1 {
                        let (f, b) = gen_cell(dx + ox, dy + oy, seed);
                        if !f.floor_walkable() || b.solid() {
                            ok = false;
                            break 'chk;
                        }
                    }
                }
                if ok && gen_cell(dx, dy, seed).0 == Block::Grass {
                    return vec2(dx as f32 + 0.5, dy as f32 + 0.5);
                }
            }
        }
    }
    vec2(0.5, 0.5)
}

// =====================================================================
// SECAO 6 - MUNDO EM CHUNKS (chao / piso / bloco) E SALAS DE CASAS
// =====================================================================

struct Chunk {
    floor: [Block; 256],
    cover: [Block; 256], // PISO: Air = sem piso
    block: [Block; 256],
    modified: bool,
}

impl Chunk {
    fn generate(cx: i32, cy: i32, seed: u32) -> Chunk {
        let mut floor = [Block::Air; 256];
        let mut block = [Block::Air; 256];
        for ly in 0..CHUNK {
            for lx in 0..CHUNK {
                let (f, b) = gen_cell(cx * CHUNK + lx, cy * CHUNK + ly, seed);
                let i = (ly * CHUNK + lx) as usize;
                floor[i] = f;
                block[i] = b;
            }
        }
        Chunk { floor, cover: [Block::Air; 256], block, modified: false }
    }
}

struct World {
    seed: u32,
    chunks: HashMap<(i32, i32), Chunk>,
    saplings: HashMap<(i32, i32), f32>, // mudas plantadas e quanto tempo ja cresceram
}

impl World {
    fn new(seed: u32) -> World {
        World { seed, chunks: HashMap::new(), saplings: HashMap::new() }
    }

    fn split(x: i32, y: i32) -> ((i32, i32), usize) {
        (
            (x.div_euclid(CHUNK), y.div_euclid(CHUNK)),
            (y.rem_euclid(CHUNK) * CHUNK + x.rem_euclid(CHUNK)) as usize,
        )
    }

    fn ensure_chunk(&mut self, cx: i32, cy: i32) {
        let seed = self.seed;
        self.chunks
            .entry((cx, cy))
            .or_insert_with(|| Chunk::generate(cx, cy, seed));
    }

    fn ensure_area(&mut self, ccx: i32, ccy: i32, rx: i32, ry: i32) {
        for cy in (ccy - ry)..=(ccy + ry) {
            for cx in (ccx - rx)..=(ccx + rx) {
                self.ensure_chunk(cx, cy);
            }
        }
    }

    /// Como ensure_area, mas gera no maximo `budget` chunks por chamada
    /// (evita travadas: o resto vem nos proximos quadros).
    fn ensure_area_budget(&mut self, ccx: i32, ccy: i32, rx: i32, ry: i32, mut budget: usize) {
        for cy in (ccy - ry)..=(ccy + ry) {
            for cx in (ccx - rx)..=(ccx + rx) {
                if budget == 0 {
                    return;
                }
                if !self.chunks.contains_key(&(cx, cy)) {
                    self.ensure_chunk(cx, cy);
                    budget -= 1;
                }
            }
        }
    }

    /// Descarrega chunks longe que nao foram modificados (os modificados ficam).
    fn unload_far(&mut self, ccx: i32, ccy: i32, kx: i32, ky: i32) {
        self.chunks
            .retain(|&(cx, cy), ch| ch.modified || ((cx - ccx).abs() <= kx && (cy - ccy).abs() <= ky));
    }

    /// (chao, piso, bloco) de uma celula, numa unica consulta.
    fn cell(&self, x: i32, y: i32) -> (Block, Block, Block) {
        let (key, i) = World::split(x, y);
        match self.chunks.get(&key) {
            Some(c) => (c.floor[i], c.cover[i], c.block[i]),
            None => {
                let (f, b) = gen_cell(x, y, self.seed);
                (f, Block::Air, b)
            }
        }
    }

    fn set_floor(&mut self, x: i32, y: i32, b: Block) {
        let (key, i) = World::split(x, y);
        self.ensure_chunk(key.0, key.1);
        if let Some(c) = self.chunks.get_mut(&key) {
            c.floor[i] = b;
            c.modified = true;
        }
    }

    fn set_cover(&mut self, x: i32, y: i32, b: Block) {
        let (key, i) = World::split(x, y);
        self.ensure_chunk(key.0, key.1);
        if let Some(c) = self.chunks.get_mut(&key) {
            c.cover[i] = b;
            c.modified = true;
        }
    }

    fn set_block(&mut self, x: i32, y: i32, b: Block) {
        let (key, i) = World::split(x, y);
        self.ensure_chunk(key.0, key.1);
        if let Some(c) = self.chunks.get_mut(&key) {
            c.block[i] = b;
            c.modified = true;
        }
    }
}

// ---------------------------------------------------------------------
// SALA (interior de uma casa): outro "mundo", com fundo preto, de tamanho
// DOBRADO em relacao ao vao da casa por fora. Uma casa 4x2 vira 8x4.
// A grade tem uma moldura de 1 bloco (parede da casa + a porta).
// Fora da moldura e tudo Void (preto, nao anda).
// ---------------------------------------------------------------------
struct Room {
    w: i32, // vao interno livre (largura)
    h: i32,
    floor: Vec<Block>,
    cover: Vec<Block>,
    block: Vec<Block>,
    origin: (i32, i32),   // canto noroeste do vao por fora (para notar expansoes)
    door_out: (i32, i32), // porta da casa no mundo
    out_dir: (i32, i32),  // direcao de dentro para fora, atravessando a porta
    door_in: (i32, i32),  // porta da sala (por dentro)
}

impl Room {
    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < -1 || y < -1 || x > self.w || y > self.h {
            None
        } else {
            Some(((x + 1) + (y + 1) * (self.w + 2)) as usize)
        }
    }

    /// `w` e `h` sao o vao por fora; a sala por dentro tem o dobro.
    fn build(w: i32, h: i32, origin: (i32, i32), door_out: (i32, i32), out_dir: (i32, i32), base: Block) -> Room {
        let rw = w * 2;
        let rh = h * 2;
        let n = ((rw + 2) * (rh + 2)) as usize;
        let mut room = Room {
            w: rw,
            h: rh,
            floor: vec![Block::Void; n],
            cover: vec![Block::Air; n],
            block: vec![Block::Air; n],
            origin,
            door_out,
            out_dir,
            door_in: (0, 0),
        };
        for y in -1..=rh {
            for x in -1..=rw {
                if let Some(i) = room.idx(x, y) {
                    let ring = x == -1 || y == -1 || x == rw || y == rh;
                    if ring {
                        room.floor[i] = Block::Dirt;
                        room.block[i] = Block::HouseWall;
                    } else {
                        room.floor[i] = base;
                    }
                }
            }
        }
        // a porta fica na mesma parede da porta de fora, na posicao proporcional
        let lx = ((door_out.0 - origin.0) * 2).clamp(0, rw - 1);
        let ly = ((door_out.1 - origin.1) * 2).clamp(0, rh - 1);
        let door_in = match out_dir {
            (0, 1) => (lx, rh),
            (0, -1) => (lx, -1),
            (1, 0) => (rw, ly),
            _ => (-1, ly),
        };
        room.door_in = door_in;
        if let Some(i) = room.idx(door_in.0, door_in.1) {
            room.block[i] = Block::Door;
        }
        room
    }

    fn floor(&self, x: i32, y: i32) -> Block {
        self.idx(x, y).map_or(Block::Void, |i| self.floor[i])
    }
    fn cover(&self, x: i32, y: i32) -> Block {
        self.idx(x, y).map_or(Block::Air, |i| self.cover[i])
    }
    fn block(&self, x: i32, y: i32) -> Block {
        self.idx(x, y).map_or(Block::Air, |i| self.block[i])
    }
    fn set_floor(&mut self, x: i32, y: i32, b: Block) {
        if let Some(i) = self.idx(x, y) {
            self.floor[i] = b;
        }
    }
    fn set_cover(&mut self, x: i32, y: i32, b: Block) {
        if let Some(i) = self.idx(x, y) {
            self.cover[i] = b;
        }
    }
    fn set_block(&mut self, x: i32, y: i32, b: Block) {
        if let Some(i) = self.idx(x, y) {
            self.block[i] = b;
        }
    }

    /// Quando a casa e ampliada por fora, o interior cresce guardando o que voce construiu.
    fn copy_from(&mut self, old: &Room) {
        let dx = (old.origin.0 - self.origin.0) * 2;
        let dy = (old.origin.1 - self.origin.1) * 2;
        for y in 0..old.h {
            for x in 0..old.w {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < self.w && ny < self.h {
                    if let (Some(oi), Some(ni)) = (old.idx(x, y), self.idx(nx, ny)) {
                        self.floor[ni] = old.floor[oi];
                        self.cover[ni] = old.cover[oi];
                        self.block[ni] = old.block[oi];
                    }
                }
            }
        }
    }
}

/// Retangulo livre cercado por blocos (o vao de uma casa).
struct Region {
    min: (i32, i32),
    max: (i32, i32),
}


fn box_hits(walk: &dyn Fn(i32, i32) -> bool, p: Vec2) -> bool {
    let x0 = (p.x - PLAYER_HALF).floor() as i32;
    let x1 = (p.x + PLAYER_HALF).floor() as i32;
    let y0 = (p.y - PLAYER_HALF).floor() as i32;
    let y1 = (p.y + PLAYER_HALF).floor() as i32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            if !walk(x, y) {
                return true;
            }
        }
    }
    false
}

/// Move um corpo com colisao (eixo X e Y separados = desliza nas paredes).
fn move_entity(walk: &dyn Fn(i32, i32) -> bool, pos: &mut Vec2, d: Vec2) {
    let nx = vec2(pos.x + d.x, pos.y);
    if !box_hits(walk, nx) {
        pos.x = nx.x;
    }
    let ny = vec2(pos.x, pos.y + d.y);
    if !box_hits(walk, ny) {
        pos.y = ny.y;
    }
}

// =====================================================================
// SECAO 7 - INVENTARIO E ENTIDADES
// =====================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
struct ItemStack {
    item: Item,
    count: u8,
}

fn make_stack(item: Item, count: u8) -> Option<ItemStack> {
    if count == 0 {
        None
    } else {
        Some(ItemStack { item, count })
    }
}

/// Esquerdo: pega / larga / junta / troca. Direito: metade / um por vez.
/// Serve para qualquer slot (inventario ou grade de crafting).
fn click_slot(cursor: &mut Option<ItemStack>, slot: &mut Option<ItemStack>, right: bool) {
    let cur = *cursor;
    let sl = *slot;
    match (cur, sl) {
        (None, None) => {}
        (None, Some(s)) => {
            if right {
                let take = (s.count + 1) / 2;
                *cursor = make_stack(s.item, take);
                *slot = make_stack(s.item, s.count - take);
            } else {
                *cursor = Some(s);
                *slot = None;
            }
        }
        (Some(c), None) => {
            if right {
                *slot = make_stack(c.item, 1);
                *cursor = make_stack(c.item, c.count - 1);
            } else {
                *slot = Some(c);
                *cursor = None;
            }
        }
        (Some(c), Some(s)) => {
            if c.item == s.item {
                let room = c.item.max_stack() - s.count;
                let n = if right { 1u8.min(room).min(c.count) } else { room.min(c.count) };
                *slot = make_stack(s.item, s.count + n);
                *cursor = make_stack(c.item, c.count - n);
            } else {
                *slot = Some(c);
                *cursor = Some(s);
            }
        }
    }
}

/// slots 0..9 = hotbar, 9..36 = inventario
struct Inventory {
    slots: [Option<ItemStack>; 36],
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { slots: [None; 36] }
    }

    /// Adiciona itens. Retorna quantos NAO couberam.
    fn add(&mut self, item: Item, mut count: u8) -> u8 {
        let max = item.max_stack();
        for s in self.slots.iter_mut() {
            if count == 0 {
                break;
            }
            if let Some(st) = s {
                if st.item == item && st.count < max {
                    let n = (max - st.count).min(count);
                    st.count += n;
                    count -= n;
                }
            }
        }
        for s in self.slots.iter_mut() {
            if count == 0 {
                break;
            }
            if s.is_none() {
                let n = max.min(count);
                *s = Some(ItemStack { item, count: n });
                count -= n;
            }
        }
        count
    }

    fn remove_one(&mut self, slot: usize) {
        let s = &mut self.slots[slot];
        let empty = if let Some(st) = s.as_mut() {
            st.count = st.count.saturating_sub(1);
            st.count == 0
        } else {
            false
        };
        if empty {
            *s = None;
        }
    }
}

/// Em qual "lugar" o jogador esta: no mundo ou dentro de uma casa (chave = porta).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dim {
    Over,
    House((i32, i32)),
}

struct ItemDrop {
    pos: Vec2,
    vel: Vec2,
    item: Item,
    count: u8,
    age: f32,
    phase: f32,
    dim: Dim,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max_life: f32,
    color: Color,
    size: f32,
}

/// Arvore caindo: balanca, cai para um lado (dir = -1 esquerda, +1 direita) e solta madeira.
#[derive(Clone, Copy)]
struct FallingTree {
    cell: (i32, i32),
    size: usize, // 0 pequena, 1 media, 2 grande
    snowy: bool,
    dir: f32,
    t: f32,
    landed: bool,
}

struct Player {
    pos: Vec2,     // centro dos pes, em blocos
    facing: usize, // 0..8 (olhando para o mouse)
    aim: Vec2,     // vetor unitario do jogador ate o mouse
    anim_t: f32,
    walking: bool,
    interact_t: f32,
    hp: i32,
    hurt_t: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Layer {
    Block,
    Cover,
    Ground,
}

struct BreakState {
    cell: (i32, i32),
    layer: Layer,
    progress: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UiMode {
    None,
    Inventory,
    Workbench,
    Pause,
    Settings,
    Skin,
    Gallery,
}

// ---------------------------------------------------------------------
// CONFIGURACOES (menu Esc -> Configuracoes). Salvas em bio_config.txt
// ---------------------------------------------------------------------
#[derive(Clone)]
struct Settings {
    zoom: f32,        // 2..6
    height3d: f32,    // altura do efeito 3D em pixels de arte (0 = desligado)
    cam_smooth: f32,  // suavidade da camera
    show_target: bool,
    show_fps: bool,
    show_help: bool,
    particles: bool,
    tree_hurts: bool, // arvore que cai machuca
    mobile_mode: bool, // modo celular (teste): botoes na tela + mira automatica
}

impl Settings {
    fn standard() -> Settings {
        Settings {
            zoom: 3.0,
            height3d: 5.0,
            cam_smooth: 12.0,
            show_target: true,
            show_fps: false,
            show_help: true,
            particles: true,
            tree_hurts: true,
            mobile_mode: false,
        }
    }
}

fn b01(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

fn settings_to_text(s: &Settings) -> String {
    format!(
        "zoom={}\nheight3d={}\ncam_smooth={}\nshow_target={}\nshow_fps={}\nshow_help={}\nparticles={}\ntree_hurts={}\nmobile_mode={}\n",
        s.zoom,
        s.height3d,
        s.cam_smooth,
        b01(s.show_target),
        b01(s.show_fps),
        b01(s.show_help),
        b01(s.particles),
        b01(s.tree_hurts),
        b01(s.mobile_mode)
    )
}

fn parse_kv(text: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            m.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    m
}

fn settings_from_map(m: &HashMap<String, String>) -> Settings {
    let mut s = Settings::standard();
    let f = |k: &str| m.get(k).and_then(|v| v.parse::<f32>().ok());
    let b = |k: &str| m.get(k).map(|v| v == "1");
    if let Some(v) = f("zoom") {
        s.zoom = v.clamp(2.0, 10.0).round();
    }
    if let Some(v) = f("height3d") {
        s.height3d = v.clamp(0.0, 10.0).round();
    }
    if let Some(v) = f("cam_smooth") {
        s.cam_smooth = v.clamp(4.0, 30.0);
    }
    if let Some(v) = b("show_target") {
        s.show_target = v;
    }
    if let Some(v) = b("show_fps") {
        s.show_fps = v;
    }
    if let Some(v) = b("show_help") {
        s.show_help = v;
    }
    if let Some(v) = b("particles") {
        s.particles = v;
    }
    if let Some(v) = b("tree_hurts") {
        s.tree_hurts = v;
    }
    if let Some(v) = b("mobile_mode") {
        s.mobile_mode = v;
    }
    s
}

fn rgb_hex(c: [u8; 3]) -> String {
    format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

fn parse_rgb(s: &str) -> Option<[u8; 3]> {
    if s.len() != 6 || !s.is_ascii() {
        return None;
    }
    Some([
        u8::from_str_radix(&s[0..2], 16).ok()?,
        u8::from_str_radix(&s[2..4], 16).ok()?,
        u8::from_str_radix(&s[4..6], 16).ok()?,
    ])
}

/// Nome de skin seguro para virar nome de arquivo.
fn clean_name(s: &str) -> String {
    let t: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .take(16)
        .collect();
    if t.is_empty() {
        "skin".to_string()
    } else {
        t
    }
}

fn skin_to_text(s: &Skin) -> String {
    format!(
        "name={}\ngender={}\nhair={}\nskin={}\nshirt={}\npants={}\nshoes={}\neyes={}\n",
        clean_name(&s.name),
        if s.female { "f" } else { "m" },
        rgb_hex(s.hair),
        rgb_hex(s.skin),
        rgb_hex(s.shirt),
        rgb_hex(s.pants),
        rgb_hex(s.shoes),
        rgb_hex(s.eyes)
    )
}

fn skin_from_map(m: &HashMap<String, String>) -> Skin {
    let mut s = Skin::standard();
    if let Some(v) = m.get("name") {
        s.name = clean_name(v);
    }
    if let Some(v) = m.get("gender") {
        s.female = v.starts_with('f');
    }
    let keys = ["hair", "skin", "shirt", "pants", "shoes", "eyes"];
    for (i, key) in keys.iter().enumerate() {
        if let Some(c) = m.get(*key).and_then(|v| parse_rgb(v)) {
            s.set_part(i, c);
        }
    }
    s
}

/// Pasta onde o jogo guarda configuracao e skins.
/// No PC: a pasta onde esta o executavel. No Android: a pasta privada do aplicativo.
fn base_dir() -> String {
    if cfg!(target_os = "android") {
        let d = "/data/data/com.bio.game/files";
        let _ = fs::create_dir_all(d);
        d.to_string()
    } else {
        // Pasta do proprio executavel (bio.exe): funciona de qualquer lugar,
        // inclusive em emuladores de Windows no Android (Winlator / GameHub).
        match std::env::current_exe() {
            Ok(p) => match p.parent() {
                Some(d) => d.to_string_lossy().to_string(),
                None => ".".to_string(),
            },
            Err(_) => ".".to_string(),
        }
    }
}

fn config_path() -> String {
    format!("{}/{}", base_dir(), CONFIG_FILE)
}

fn skin_dir() -> String {
    format!("{}/{}", base_dir(), SKIN_DIR)
}

fn load_config() -> (Settings, Skin) {
    match fs::read_to_string(config_path()) {
        Ok(t) => {
            let m = parse_kv(&t);
            (settings_from_map(&m), skin_from_map(&m))
        }
        Err(_) => (Settings::standard(), Skin::standard()),
    }
}

fn save_config(s: &Settings, k: &Skin) {
    let text = format!("{}{}", settings_to_text(s), skin_to_text(k));
    let _ = fs::write(config_path(), text);
}

/// Exporta a skin para skins/<nome>.bioskin (arquivo de texto que voce pode compartilhar).
fn export_skin(skin: &Skin) -> Result<String, String> {
    let name = clean_name(&skin.name);
    fs::create_dir_all(skin_dir()).map_err(|e| e.to_string())?;
    let path = format!("{}/{}.bioskin", skin_dir(), name);
    fs::write(&path, skin_to_text(skin)).map_err(|e| e.to_string())?;
    Ok(path)
}

fn import_skin(name: &str) -> Option<Skin> {
    let t = fs::read_to_string(format!("{}/{}.bioskin", skin_dir(), name)).ok()?;
    let m = parse_kv(&t);
    if !m.contains_key("hair") {
        return None;
    }
    Some(skin_from_map(&m))
}

fn list_skins() -> Vec<String> {
    let mut v = Vec::new();
    if let Ok(rd) = fs::read_dir(skin_dir()) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) == Some("bioskin") {
                if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                    v.push(stem.to_string());
                }
            }
        }
    }
    v.sort();
    v
}

// =====================================================================
// SECAO 8 - CONTROLES
//  Toda a leitura de teclado/mouse fica AQUI. O jogo so olha a struct
//  `Controls`. Para ter toque na tela no futuro, basta preencher os
//  mesmos campos a partir de joystick virtual / botoes na tela.
// =====================================================================

struct Controls {
    move_axis: Vec2,   // -1..1 (y positivo = baixo)
    pointer: Vec2,     // posicao do mouse/toque em pixels
    attack_down: bool, // botao esquerdo segurado
    attack_pressed: bool,
    use_down: bool, // botao direito segurado
    use_pressed: bool,
    place_floor: bool, // Shift: colocar como piso
    slot_select: Option<usize>,
    scroll: i32,
    toggle_inventory: bool,
    cancel: bool,
    drop_item: bool,
    zoom_delta: i32,
    toggle_debug: bool,
    toggle_gallery: bool,
    test_hurt: bool,
    toggle_help: bool, // tecla H: lista de controles
    typed: Vec<char>, // letras digitadas neste quadro (campo de nome da skin)
    backspace: bool,
    aim_override: Option<Vec2>, // toque: direcao de mira ja calculada
}

fn read_controls() -> Controls {
    let mut mv = Vec2::ZERO;
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        mv.x -= 1.0;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        mv.x += 1.0;
    }
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        mv.y -= 1.0;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        mv.y += 1.0;
    }

    let slot_keys = [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ];
    let mut slot_select = None;
    for (i, k) in slot_keys.iter().enumerate() {
        if is_key_pressed(*k) {
            slot_select = Some(i);
        }
    }

    let (_, wy) = mouse_wheel();
    let scroll = if wy > 0.0 {
        -1
    } else if wy < 0.0 {
        1
    } else {
        0
    };

    let mut zoom_delta = 0;
    if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
        zoom_delta -= 1;
    }
    if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
        zoom_delta += 1;
    }

    let mut typed = Vec::new();
    while let Some(ch) = get_char_pressed() {
        typed.push(ch);
    }

    let (mx, my) = mouse_position();
    Controls {
        move_axis: mv,
        pointer: vec2(mx, my),
        attack_down: is_mouse_button_down(MouseButton::Left),
        attack_pressed: is_mouse_button_pressed(MouseButton::Left),
        use_down: is_mouse_button_down(MouseButton::Right),
        use_pressed: is_mouse_button_pressed(MouseButton::Right),
        place_floor: is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),
        slot_select,
        scroll,
        toggle_inventory: is_key_pressed(KeyCode::E),
        cancel: is_key_pressed(KeyCode::Escape),
        drop_item: is_key_pressed(KeyCode::Q),
        zoom_delta,
        toggle_debug: is_key_pressed(KeyCode::F3),
        toggle_gallery: is_key_pressed(KeyCode::F1),
        test_hurt: is_key_pressed(KeyCode::F4),
        toggle_help: is_key_pressed(KeyCode::H),
        typed,
        backspace: is_key_pressed(KeyCode::Backspace),
        aim_override: None,
    }
}

// =====================================================================
// SECAO 9 - O JOGO (estado + logica)
// =====================================================================

struct Game {
    world: World,
    rooms: HashMap<(i32, i32), Room>,
    dim: Dim,
    player: Player,
    inv: Inventory,
    cursor_stack: Option<ItemStack>,
    craft: [Option<ItemStack>; 9],
    craft_n: usize,
    craft_out: Option<(Item, u8)>,
    recipes: Vec<Recipe>,
    drops: Vec<ItemDrop>,
    particles: Vec<Particle>,
    falling: Vec<FallingTree>,
    atlas: Texture2D,
    human: SpriteSheet,
    trees: Vec<TreeTex>,
    cam: Vec2,
    settings: Settings,
    skin: Skin,
    skin_dirty: bool,
    skin_part: usize,
    name_edit: bool,
    skin_list: Vec<String>,
    ui: UiMode,
    ui_active: Option<u32>,
    selected: usize,
    breaking: Option<BreakState>,
    target: (i32, i32),
    place_cd: f32,
    time: f32,
    pointer: Vec2,
    spawn: Vec2,
    show_debug: bool,
    msg: Option<(String, f32)>,
    help_t: f32,
    last_cell: (i32, i32),
    fade: f32,
    wait_release: bool,
    touch_ui: bool,
    touch: TouchState,
    fit_k: f32,
    show_controls: bool,
}

#[derive(Clone, Copy)]
enum DrawKind {
    Block(i32, i32, Block),
    Falling(usize),
    Drop(usize),
    Player,
}

impl Game {
    fn new() -> Game {
        let (mut settings, skin) = load_config();
        // primeira vez no celular: zoom do mundo proporcional ao tamanho da tela
        if cfg!(target_os = "android") && fs::metadata(config_path()).is_err() {
            settings.zoom = (screen_width().min(screen_height()) / 180.0).round().clamp(3.0, 10.0);
        }
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1234567);
        rand::srand(t);
        let seed = FIXED_SEED.unwrap_or((t ^ (t >> 32)) as u32);

        let spawn = find_spawn(seed);
        let mut world = World::new(seed);
        let ccx = (spawn.x.floor() as i32).div_euclid(CHUNK);
        let ccy = (spawn.y.floor() as i32).div_euclid(CHUNK);
        world.ensure_area(ccx, ccy, 1, 1);

        let mut inv = Inventory::new();
        if STARTER_KIT {
            let kit: [(Item, u8); 6] = [
                (Item::Tool(ToolKind::Axe, Tier::Wood), 1),
                (Item::Tool(ToolKind::Pickaxe, Tier::Wood), 1),
                (Item::Tool(ToolKind::Shovel, Tier::Wood), 1),
                (Item::Planks, 32),
                (Item::Door, 3),
                (Item::Workbench, 1),
            ];
            for (i, (it, n)) in kit.iter().enumerate() {
                inv.slots[i] = Some(ItemStack { item: *it, count: *n });
            }
        }

        let human = SpriteSheet {
            tex: build_human_sheet(&skin),
            fw: HUMAN_W as f32,
            fh: HUMAN_H as f32,
        };

        Game {
            world,
            rooms: HashMap::new(),
            dim: Dim::Over,
            player: Player {
                pos: spawn,
                facing: 2,
                aim: vec2(0.0, 1.0),
                anim_t: 0.0,
                walking: false,
                interact_t: 0.0,
                hp: MAX_HP,
                hurt_t: 0.0,
            },
            inv,
            cursor_stack: None,
            craft: [None; 9],
            craft_n: 2,
            craft_out: None,
            recipes: build_recipes(),
            drops: Vec::new(),
            particles: Vec::new(),
            falling: Vec::new(),
            atlas: build_atlas(),
            human,
            trees: build_trees(),
            cam: spawn,
            settings,
            skin,
            skin_dirty: false,
            skin_part: 0,
            name_edit: false,
            skin_list: list_skins(),
            ui: UiMode::None,
            ui_active: None,
            selected: 0,
            breaking: None,
            target: (0, 0),
            place_cd: 0.0,
            time: 0.0,
            pointer: Vec2::ZERO,
            spawn,
            show_debug: false,
            msg: None,
            help_t: 16.0,
            last_cell: (spawn.x.floor() as i32, spawn.y.floor() as i32),
            fade: 0.0,
            wait_release: false,
            touch_ui: cfg!(target_os = "android"),
            touch: TouchState::new(),
            fit_k: 1.0,
            show_controls: false,
        }
    }

    // ----- utilidades de tela -----
    fn ts(&self) -> f32 {
        TILE as f32 * self.settings.zoom
    }
    fn cam_px(&self) -> Vec2 {
        (self.cam * self.ts()).round()
    }
    fn half(&self) -> Vec2 {
        vec2((screen_width() / 2.0).floor(), (screen_height() / 2.0).floor())
    }
    fn to_screen(&self, p: Vec2) -> Vec2 {
        p * self.ts() - self.cam_px() + self.half()
    }
    fn cell_screen(&self, x: i32, y: i32) -> Vec2 {
        self.to_screen(vec2(x as f32, y as f32))
    }
    fn held(&self) -> Option<Item> {
        self.inv.slots[self.selected].map(|s| s.item)
    }
    fn say(&mut self, s: &str, secs: f32) {
        self.msg = Some((s.to_string(), secs));
    }
    fn paused(&self) -> bool {
        matches!(
            self.ui,
            UiMode::Pause | UiMode::Settings | UiMode::Skin | UiMode::Gallery
        )
    }

    // ----- acesso ao mundo da dimensao atual (mundo ou interior de casa) -----
    fn g_cell(&self, x: i32, y: i32) -> (Block, Block, Block) {
        match self.dim {
            Dim::Over => self.world.cell(x, y),
            Dim::House(k) => match self.rooms.get(&k) {
                Some(r) => (r.floor(x, y), r.cover(x, y), r.block(x, y)),
                None => (Block::Void, Block::Air, Block::Air),
            },
        }
    }
    fn g_floor(&self, x: i32, y: i32) -> Block {
        self.g_cell(x, y).0
    }
    fn g_cover(&self, x: i32, y: i32) -> Block {
        self.g_cell(x, y).1
    }
    fn g_block(&self, x: i32, y: i32) -> Block {
        self.g_cell(x, y).2
    }
    fn g_set_floor(&mut self, x: i32, y: i32, b: Block) {
        match self.dim {
            Dim::Over => self.world.set_floor(x, y, b),
            Dim::House(k) => {
                if let Some(r) = self.rooms.get_mut(&k) {
                    r.set_floor(x, y, b);
                }
            }
        }
    }
    fn g_set_cover(&mut self, x: i32, y: i32, b: Block) {
        match self.dim {
            Dim::Over => self.world.set_cover(x, y, b),
            Dim::House(k) => {
                if let Some(r) = self.rooms.get_mut(&k) {
                    r.set_cover(x, y, b);
                }
            }
        }
    }
    fn g_set_block(&mut self, x: i32, y: i32, b: Block) {
        match self.dim {
            Dim::Over => self.world.set_block(x, y, b),
            Dim::House(k) => {
                if let Some(r) = self.rooms.get_mut(&k) {
                    r.set_block(x, y, b);
                }
            }
        }
    }
    fn g_walkable(&self, x: i32, y: i32) -> bool {
        let (f, cv, b) = self.g_cell(x, y);
        if b.solid() {
            return false;
        }
        cv != Block::Air || f.floor_walkable()
    }

    fn player_overlaps_cell(&self, cx: i32, cy: i32) -> bool {
        let p = self.player.pos;
        p.x + PLAYER_HALF > cx as f32
            && p.x - PLAYER_HALF < (cx + 1) as f32
            && p.y + PLAYER_HALF > cy as f32
            && p.y - PLAYER_HALF < (cy + 1) as f32
    }

    // ----- UPDATE -----
    fn update(&mut self, c: &Controls, dt: f32) {
        self.time += dt;
        self.pointer = c.pointer;
        self.help_t = (self.help_t - dt).max(0.0);
        self.fade = (self.fade - dt * 2.5).max(0.0);
        if self.wait_release && !c.attack_down && !c.use_down {
            self.wait_release = false;
        }
        let expired = if let Some(m) = self.msg.as_mut() {
            m.1 -= dt;
            m.1 <= 0.0
        } else {
            false
        };
        if expired {
            self.msg = None;
        }

        if c.cancel {
            if self.show_controls {
                self.show_controls = false; // Esc fecha primeiro a lista de controles
            } else {
                self.handle_cancel();
            }
        }
        // digitando o nome da skin: teclas do jogo ficam desligadas
        if self.ui == UiMode::Skin && self.name_edit {
            return;
        }
        if c.toggle_gallery {
            if self.ui == UiMode::Gallery {
                self.ui = UiMode::None;
            } else {
                self.close_container();
                self.ui = UiMode::Gallery;
            }
        }
        if c.toggle_debug {
            self.show_debug = !self.show_debug;
        }
        if c.toggle_help {
            self.show_controls = !self.show_controls;
        }
        if self.paused() {
            return;
        }

        if c.toggle_inventory {
            match self.ui {
                UiMode::None => self.open_container(2),
                UiMode::Inventory | UiMode::Workbench => self.close_container(),
                _ => {}
            }
        }
        if c.zoom_delta != 0 {
            self.settings.zoom = (self.settings.zoom + c.zoom_delta as f32).clamp(2.0, 10.0);
        }
        if c.test_hurt {
            self.hurt(1, false);
        }
        let in_game = self.ui == UiMode::None;
        if let Some(s) = c.slot_select {
            self.selected = s;
        }
        if c.scroll != 0 && in_game {
            self.selected = (self.selected as i32 + c.scroll).rem_euclid(9) as usize;
        }

        self.player.hurt_t = (self.player.hurt_t - dt).max(0.0);
        self.player.interact_t = (self.player.interact_t - dt).max(0.0);
        self.place_cd = (self.place_cd - dt).max(0.0);

        let atk_down = c.attack_down && !self.wait_release;
        let atk_pressed = c.attack_pressed && !self.wait_release;
        let use_down = c.use_down && !self.wait_release;

        // --- movimento (8 direcoes, diagonal normalizada)
        let axis = if in_game { c.move_axis } else { Vec2::ZERO };
        let delta = axis.normalize_or_zero() * PLAYER_SPEED * dt;
        let before = self.player.pos;
        let mut p = before;
        {
            let this = &*self;
            move_entity(&|x, y| this.g_walkable(x, y), &mut p, delta);
        }
        self.player.pos = p;
        let moved = (p - before).length() > 0.0001;
        self.player.walking = moved;
        if moved {
            self.player.anim_t += dt;
        } else {
            self.player.anim_t = 0.0;
        }

        // --- portas / casas
        let cell = (p.x.floor() as i32, p.y.floor() as i32);
        if cell != self.last_cell {
            let prev = self.last_cell;
            self.last_cell = cell;
            if self.g_block(cell.0, cell.1) == Block::Door {
                self.step_on_door(prev, cell);
            }
        }

        // --- camera suave + chunks
        let k = 1.0 - (-self.settings.cam_smooth * dt).exp();
        self.cam += (self.player.pos - self.cam) * k;
        if self.dim == Dim::Over {
            self.update_chunks();
        }

        // --- mira e acoes
        if in_game {
            let torso = self.to_screen(self.player.pos + vec2(0.0, -0.45));
            if let Some(a) = c.aim_override {
                self.player.aim = a; // toque: a mira ja vem pronta
                self.player.facing = dir8_from_vec(a);
            } else {
                let d = c.pointer - torso;
                if d.length() > 6.0 {
                    self.player.aim = d.normalize();
                    self.player.facing = dir8_from_vec(self.player.aim);
                }
            }
            let tp = self.player.pos + self.player.aim * REACH;
            self.target = (tp.x.floor() as i32, tp.y.floor() as i32);

            self.update_breaking(atk_down, atk_pressed, dt);
            if use_down && self.place_cd <= 0.0 {
                self.do_use(c.place_floor);
                self.place_cd = 0.22;
            }
            if c.drop_item {
                self.drop_selected();
            }
        } else {
            self.breaking = None;
            if c.attack_pressed {
                self.container_click(false);
            }
            if c.use_pressed {
                self.container_click(true);
            }
        }

        self.update_drops(dt);
        self.update_particles(dt);
        self.update_falling(dt);
        if self.dim == Dim::Over {
            self.update_saplings(dt);
        }
    }

    fn handle_cancel(&mut self) {
        match self.ui {
            UiMode::None => self.ui = UiMode::Pause,
            UiMode::Inventory | UiMode::Workbench => self.close_container(),
            UiMode::Pause => self.ui = UiMode::None,
            UiMode::Settings | UiMode::Skin | UiMode::Gallery => self.leave_submenu(),
        }
    }

    fn leave_submenu(&mut self) {
        self.name_edit = false;
        self.skin_dirty = false;
        save_config(&self.settings, &self.skin);
        self.ui = UiMode::Pause;
    }

    fn update_chunks(&mut self) {
        let ts = self.ts();
        let rx = ((screen_width() / ts / 2.0) / CHUNK as f32).ceil() as i32 + 1;
        let ry = ((screen_height() / ts / 2.0) / CHUNK as f32).ceil() as i32 + 1;
        let ccx = (self.cam.x.floor() as i32).div_euclid(CHUNK);
        let ccy = (self.cam.y.floor() as i32).div_euclid(CHUNK);
        self.world.ensure_area_budget(ccx, ccy, rx, ry, 6);
        self.world.unload_far(ccx, ccy, rx + 2, ry + 2);
    }

    fn hurt(&mut self, n: i32, force: bool) {
        if self.player.hurt_t > 0.0 && !force {
            return;
        }
        self.player.hp -= n;
        self.player.hurt_t = 0.6;
        if self.player.hp <= 0 {
            self.player.hp = MAX_HP;
            self.dim = Dim::Over;
            self.player.pos = self.spawn;
            self.cam = self.spawn;
            self.last_cell = (self.spawn.x.floor() as i32, self.spawn.y.floor() as i32);
            self.breaking = None;
            self.fade = 1.0;
            self.say("Voce morreu! Voltou ao ponto inicial (itens mantidos)", 3.0);
        }
    }

}

// =====================================================================
// SECAO 9b - ACOES: quebrar, colocar, arvores, crafting, casas
// =====================================================================
impl Game {
    // ----- quebrar (botao esquerdo, segurando) -----
    // Prioridade: BLOCO -> PISO -> CHAO
    fn update_breaking(&mut self, down: bool, pressed: bool, dt: f32) {
        if !down {
            self.breaking = None;
            return;
        }
        if pressed {
            self.player.interact_t = 0.25;
        }
        let (cx, cy) = self.target;
        let (f, cv, b) = self.g_cell(cx, cy);
        let (layer, blk) = if b != Block::Air {
            (Layer::Block, b)
        } else if cv != Block::Air {
            (Layer::Cover, cv)
        } else {
            (Layer::Ground, f)
        };
        let held = self.held();
        let in_room = self.dim != Dim::Over;
        let blocked = match layer {
            // nao da para cavar dentro de casa nem o chao onde voce esta pisando
            Layer::Ground => in_room || self.player_overlaps_cell(cx, cy),
            Layer::Block => in_room && blk == Block::Door,
            Layer::Cover => false,
        };
        let time_needed = if blocked { None } else { break_time(blk, held) };
        let Some(t) = time_needed else {
            self.breaking = None;
            return;
        };

        let same = match &self.breaking {
            Some(s) => s.cell == (cx, cy) && s.layer == layer,
            None => false,
        };
        if !same {
            self.breaking = Some(BreakState { cell: (cx, cy), layer, progress: 0.0 });
        }
        let done = {
            let s = self.breaking.as_mut().unwrap();
            s.progress += if t <= 0.0 { 1.0 } else { dt / t };
            s.progress >= 1.0
        };
        if self.settings.particles && rand::gen_range(0.0f32, 1.0f32) < dt * 14.0 {
            self.spawn_particle(vec2(cx as f32 + 0.5, cy as f32 + 0.5), blk.color());
        }
        if done {
            self.breaking = None;
            self.finish_break(cx, cy, layer, blk, held);
        }
    }

    fn finish_break(&mut self, cx: i32, cy: i32, layer: Layer, b: Block, held: Option<Item>) {
        match layer {
            Layer::Block => {
                if b.is_tree() {
                    self.start_tree_fall(cx, cy, b);
                    return;
                }
                self.g_set_block(cx, cy, Block::Air);
                if b == Block::Sapling && self.dim == Dim::Over {
                    self.world.saplings.remove(&(cx, cy));
                }
            }
            Layer::Cover => self.g_set_cover(cx, cy, Block::Air),
            // Fase 2: este buraco levara ao subterraneo
            Layer::Ground => self.g_set_floor(cx, cy, Block::Hole),
        }
        let center = vec2(cx as f32 + 0.5, cy as f32 + 0.5);
        if self.settings.particles {
            for _ in 0..8 {
                self.spawn_particle(center, b.color());
            }
        }
        if let Some((item, n)) = drop_for(b, held) {
            let vel = vec2(rand::gen_range(-1.5f32, 1.5f32), rand::gen_range(-1.5f32, 1.5f32));
            self.spawn_drop(center, vel, item, n, 0.0);
        }
    }

    // ----- arvores -----
    fn start_tree_fall(&mut self, cx: i32, cy: i32, b: Block) {
        let snowy = self.g_floor(cx, cy) == Block::Snow;
        self.g_set_block(cx, cy, Block::Air);
        let dir = if rand::gen_range(0i32, 2i32) == 0 { -1.0 } else { 1.0 };
        self.falling.push(FallingTree {
            cell: (cx, cy),
            size: b.tree_size(),
            snowy,
            dir,
            t: 0.0,
            landed: false,
        });
    }

    fn update_falling(&mut self, dt: f32) {
        let mut i = 0;
        while i < self.falling.len() {
            self.falling[i].t += dt;
            let ft = self.falling[i];
            if !ft.landed && ft.t >= TREE_WARN + TREE_FALL {
                self.falling[i].landed = true;
                self.tree_impact(ft);
            }
            if ft.t > TREE_WARN + TREE_FALL + TREE_LINGER {
                self.falling.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// A arvore bateu no chao: machuca quem estiver embaixo e solta madeira (quantidade aleatoria).
    fn tree_impact(&mut self, ft: FallingTree) {
        let base = vec2(ft.cell.0 as f32 + 0.5, ft.cell.1 as f32 + 0.85);
        let len = [1.7f32, 2.3, 2.9][ft.size];
        let x_end = base.x + ft.dir * len;
        let x0 = base.x.min(x_end);
        let x1 = base.x.max(x_end);
        let y0 = base.y - 0.8;
        let y1 = base.y + 0.5;

        if self.dim == Dim::Over && self.settings.tree_hurts {
            let p = self.player.pos;
            if p.x + PLAYER_HALF > x0 && p.x - PLAYER_HALF < x1 && p.y + PLAYER_HALF > y0 && p.y - PLAYER_HALF < y1 {
                let dmg = [4, 8, 12][ft.size];
                self.hurt(dmg, true);
                self.say("A arvore caiu em cima de voce!", 2.0);
            }
        }

        let logs = match ft.size {
            0 => rand::gen_range(1i32, 3i32),
            1 => rand::gen_range(2i32, 5i32),
            _ => rand::gen_range(4i32, 7i32),
        };
        let leaves = match ft.size {
            0 => rand::gen_range(0i32, 3i32),
            1 => rand::gen_range(1i32, 4i32),
            _ => rand::gen_range(2i32, 5i32),
        };
        let mut saplings = 0i32;
        if rand::gen_range(0.0f32, 1.0f32) < 0.55 {
            saplings += 1;
        }
        if ft.size == 2 && rand::gen_range(0.0f32, 1.0f32) < 0.4 {
            saplings += 1;
        }

        let lots = [(Item::Log, logs), (Item::Leaves, leaves), (Item::Sapling, saplings)];
        for (item, n) in lots.iter() {
            for _ in 0..*n {
                let f = 0.25 + 0.75 * rand::gen_range(0.0f32, 1.0f32);
                let pos = vec2(base.x + ft.dir * len * f, base.y + rand::gen_range(-0.3f32, 0.3f32));
                let vel = vec2(rand::gen_range(-0.6f32, 0.6f32), rand::gen_range(-0.6f32, 0.6f32));
                self.spawn_drop(pos, vel, *item, 1, 0.0);
            }
        }
        if self.settings.particles {
            let leaf = if ft.snowy { hex(0xcfe6da) } else { hex(0x3f8a2f) };
            for _ in 0..18 {
                let f = rand::gen_range(0.0f32, 1.0f32);
                self.spawn_particle(vec2(base.x + ft.dir * len * f, base.y - 0.2), leaf);
            }
        }
    }

    fn update_saplings(&mut self, dt: f32) {
        let mut grown: Vec<(i32, i32)> = Vec::new();
        for (k, t) in self.world.saplings.iter_mut() {
            *t += dt;
            if *t >= SAPLING_GROW_SECS {
                grown.push(*k);
            }
        }
        for k in grown {
            if self.dim == Dim::Over && self.player_overlaps_cell(k.0, k.1) {
                continue; // espera o jogador sair de cima
            }
            self.world.saplings.remove(&k);
            if self.world.cell(k.0, k.1).2 == Block::Sapling {
                self.world.set_block(k.0, k.1, Block::TreeS);
            }
        }
    }

    // ----- colocar / interagir (botao direito) -----
    fn do_use(&mut self, place_floor: bool) {
        self.player.interact_t = 0.3; // sem item: so faz o gesto de apontar
        let (cx, cy) = self.target;
        let (f, cv, top) = self.g_cell(cx, cy);
        if top == Block::Workbench {
            self.open_container(3);
            return;
        }
        let Some(item) = self.held() else {
            return;
        };
        let Some(pl) = item.placement() else {
            return;
        };
        if f == Block::Void || top.solid() {
            return;
        }
        match pl {
            Placement::Floor(b) => {
                if place_floor {
                    if cv != Block::Air {
                        return;
                    }
                    self.g_set_cover(cx, cy, b);
                } else {
                    if cv != Block::Air || f == b {
                        return;
                    }
                    self.g_set_floor(cx, cy, b);
                    if top.is_deco() {
                        self.g_set_block(cx, cy, Block::Air);
                    }
                }
                self.inv.remove_one(self.selected);
            }
            Placement::Solid(b) => {
                // piso: Shift, ou automatico sobre buraco/agua (ponte e tampa de buraco)
                if let Some(cb) = item.cover_block() {
                    let bridge = matches!(f, Block::Hole | Block::Water) && cv == Block::Air;
                    if place_floor || bridge {
                        if cv != Block::Air {
                            return;
                        }
                        self.g_set_cover(cx, cy, cb);
                        self.inv.remove_one(self.selected);
                        return;
                    }
                }
                if b.solid() && self.player_overlaps_cell(cx, cy) {
                    return;
                }
                self.g_set_block(cx, cy, b);
                self.inv.remove_one(self.selected);
            }
            Placement::Deco(b) => {
                if top != Block::Air || cv != Block::Air || !matches!(f, Block::Grass | Block::Dirt | Block::Snow) {
                    return;
                }
                if b == Block::Sapling {
                    if self.dim != Dim::Over {
                        return;
                    }
                    self.world.saplings.insert((cx, cy), 0.0);
                }
                self.g_set_block(cx, cy, b);
                self.inv.remove_one(self.selected);
            }
        }
    }

    fn drop_selected(&mut self) {
        if let Some(st) = self.inv.slots[self.selected] {
            self.inv.remove_one(self.selected);
            let p = self.player.pos + self.player.aim * 0.5;
            let v = self.player.aim * 5.0;
            self.spawn_drop(p, v, st.item, 1, -1.5);
            self.player.interact_t = 0.2;
        }
    }

    fn spawn_drop(&mut self, pos: Vec2, vel: Vec2, item: Item, count: u8, age: f32) {
        self.drops.push(ItemDrop {
            pos,
            vel,
            item,
            count,
            age,
            phase: rand::gen_range(0.0f32, 6.28f32),
            dim: self.dim,
        });
    }

    fn spawn_particle(&mut self, pos: Vec2, color: Color) {
        let a = rand::gen_range(0.0f32, PI * 2.0);
        let s = rand::gen_range(0.8f32, 2.8f32);
        let off = vec2(rand::gen_range(-0.25f32, 0.25f32), rand::gen_range(-0.25f32, 0.25f32));
        self.particles.push(Particle {
            pos: pos + off,
            vel: vec2(a.cos(), a.sin()) * s,
            life: 0.6,
            max_life: 0.6,
            color,
            size: rand::gen_range(0.06f32, 0.12f32),
        });
    }

    fn update_drops(&mut self, dt: f32) {
        let pp = self.player.pos;
        let dim = self.dim;
        let mut i = 0;
        while i < self.drops.len() {
            let mut remove = false;
            {
                let d = &mut self.drops[i];
                d.age += dt;
                if d.dim == dim {
                    d.pos += d.vel * dt;
                    d.vel *= (1.0 - 5.0 * dt).max(0.0);
                    let to = pp - d.pos;
                    let dist = to.length();
                    if d.age > 0.5 && dist < 1.8 {
                        d.pos += to.normalize_or_zero() * 7.0 * dt;
                    }
                    if d.age > 0.4 && dist < 0.55 {
                        let left = self.inv.add(d.item, d.count);
                        if left == 0 {
                            remove = true;
                        } else {
                            d.count = left;
                        }
                    }
                }
                if d.age > 300.0 {
                    remove = true;
                }
            }
            if remove {
                self.drops.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn update_particles(&mut self, dt: f32) {
        self.particles.retain_mut(|p| {
            p.life -= dt;
            p.pos += p.vel * dt;
            p.vel *= (1.0 - 4.0 * dt).max(0.0);
            p.life > 0.0
        });
    }

    // ----- inventario + crafting -----
    fn open_container(&mut self, n: usize) {
        self.craft_n = n;
        self.ui = if n == 3 { UiMode::Workbench } else { UiMode::Inventory };
        self.breaking = None;
        self.craft_update();
    }

    /// Fecha inventario/bancada e devolve tudo (grade de crafting e item do cursor).
    fn close_container(&mut self) {
        if !matches!(self.ui, UiMode::Inventory | UiMode::Workbench) {
            return;
        }
        self.ui = UiMode::None;
        let mut back: Vec<ItemStack> = Vec::new();
        if let Some(st) = self.cursor_stack.take() {
            back.push(st);
        }
        for i in 0..9 {
            if let Some(st) = self.craft[i].take() {
                back.push(st);
            }
        }
        for st in back {
            let left = self.inv.add(st.item, st.count);
            if left > 0 {
                let p = self.player.pos;
                let v = self.player.aim * 3.0;
                self.spawn_drop(p, v, st.item, left, -1.5);
            }
        }
        self.craft_out = None;
    }

    fn craft_update(&mut self) {
        self.craft_out = match_recipe(&self.recipes, &self.craft, self.craft_n);
    }

    /// Pega o resultado da receita: consome 1 de cada ingrediente.
    fn craft_take(&mut self) {
        let Some((item, n)) = self.craft_out else {
            return;
        };
        let can = match self.cursor_stack {
            None => true,
            Some(c) => c.item == item && c.count as u32 + n as u32 <= item.max_stack() as u32,
        };
        if !can {
            return;
        }
        self.cursor_stack = match self.cursor_stack {
            None => make_stack(item, n),
            Some(c) => make_stack(item, c.count + n),
        };
        for i in 0..self.craft_n * self.craft_n {
            if let Some(st) = self.craft[i] {
                self.craft[i] = make_stack(st.item, st.count - 1);
            }
        }
        self.craft_update();
    }

    fn container_click(&mut self, right: bool) {
        for idx in 0..36 {
            if self.inv_slot_rect(idx).contains(self.ptr()) {
                click_slot(&mut self.cursor_stack, &mut self.inv.slots[idx], right);
                return;
            }
        }
        for i in 0..self.craft_n * self.craft_n {
            if self.craft_slot_rect(i).contains(self.ptr()) {
                click_slot(&mut self.cursor_stack, &mut self.craft[i], right);
                self.craft_update();
                return;
            }
        }
        if self.craft_out_rect().contains(self.ptr()) {
            self.craft_take();
        }
    }

    // layout compartilhado entre clique e desenho
    fn container_panel(&self) -> Rect {
        let step = UI_SLOT + UI_GAP;
        let craft_h = self.craft_n as f32 * step + 20.0;
        let w = 9.0 * UI_SLOT + 8.0 * UI_GAP + 2.0 * UI_PAD;
        let h = 40.0 + craft_h + 4.0 * UI_SLOT + 3.0 * UI_GAP + 14.0 + 2.0 * UI_PAD;
        Rect::new((self.ui_dims().0 - w) / 2.0, (self.ui_dims().1 - h) / 2.0, w, h)
    }

    fn inv_slot_rect(&self, idx: usize) -> Rect {
        let panel = self.container_panel();
        let step = UI_SLOT + UI_GAP;
        let craft_h = self.craft_n as f32 * step + 20.0;
        let (col, row_y) = if idx < 9 {
            (idx, 3.0 * step + 14.0) // hotbar fica embaixo
        } else {
            let k = idx - 9;
            (k % 9, (k / 9) as f32 * step)
        };
        Rect::new(
            panel.x + UI_PAD + col as f32 * step,
            panel.y + UI_PAD + 40.0 + craft_h + row_y,
            UI_SLOT,
            UI_SLOT,
        )
    }

    fn craft_origin(&self) -> Vec2 {
        let panel = self.container_panel();
        let step = UI_SLOT + UI_GAP;
        let total = self.craft_n as f32 * step + 70.0 + UI_SLOT;
        vec2(panel.x + (panel.w - total) / 2.0, panel.y + UI_PAD + 40.0 + 6.0)
    }

    fn craft_slot_rect(&self, i: usize) -> Rect {
        let o = self.craft_origin();
        let step = UI_SLOT + UI_GAP;
        let n = self.craft_n;
        Rect::new(o.x + (i % n) as f32 * step, o.y + (i / n) as f32 * step, UI_SLOT, UI_SLOT)
    }

    fn craft_out_rect(&self) -> Rect {
        let o = self.craft_origin();
        let step = UI_SLOT + UI_GAP;
        let n = self.craft_n as f32;
        Rect::new(o.x + n * step + 70.0, o.y + (n * step - UI_GAP - UI_SLOT) / 2.0, UI_SLOT, UI_SLOT)
    }

    // ----- portas e casas -----

    /// A porta esta "aberta" (visual) quando o jogador esta perto.
    fn door_near(&self, wx: i32, wy: i32) -> bool {
        let c = vec2(wx as f32 + 0.5, wy as f32 + 0.5);
        (self.player.pos - c).length() < 1.5
    }

    /// Procura um retangulo livre totalmente cercado por blocos/portas a partir de `start`.
    fn find_region(&self, start: (i32, i32)) -> Option<Region> {
        let blocked = |x: i32, y: i32| {
            let b = self.world.cell(x, y).2;
            b.solid() || b == Block::Door
        };
        if blocked(start.0, start.1) {
            return None;
        }
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        seen.insert(start);
        q.push_back(start);
        let (mut minx, mut miny, mut maxx, mut maxy) = (start.0, start.1, start.0, start.1);
        while let Some((x, y)) = q.pop_front() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (x + dx, y + dy);
                if seen.contains(&n) || blocked(n.0, n.1) {
                    continue;
                }
                seen.insert(n);
                minx = minx.min(n.0);
                miny = miny.min(n.1);
                maxx = maxx.max(n.0);
                maxy = maxy.max(n.1);
                if maxx - minx + 1 > HOUSE_MAX || maxy - miny + 1 > HOUSE_MAX {
                    return None; // grande demais / aberto
                }
                q.push_back(n);
            }
        }
        let w = maxx - minx + 1;
        let h = maxy - miny + 1;
        if (w * h) as usize != seen.len() {
            return None; // nao e um retangulo
        }
        Some(Region { min: (minx, miny), max: (maxx, maxy) })
    }

    fn step_on_door(&mut self, prev: (i32, i32), cell: (i32, i32)) {
        match self.dim {
            Dim::Over => {
                let d = (cell.0 - prev.0, cell.1 - prev.1);
                if d.0.abs() + d.1.abs() != 1 {
                    return;
                }
                let ahead = (cell.0 + d.0, cell.1 + d.1);
                if let Some(region) = self.find_region(ahead) {
                    self.enter_house(cell, d, region);
                }
            }
            Dim::House(key) => {
                let exit = match self.rooms.get(&key) {
                    Some(r) if r.door_in == cell => Some((r.door_out, r.out_dir)),
                    _ => None,
                };
                if let Some((door_out, out_dir)) = exit {
                    self.exit_house(door_out, out_dir);
                }
            }
        }
    }

    /// Entra na casa: o interior tem o DOBRO do tamanho do vao (4x2 -> 8x4).
    fn enter_house(&mut self, door: (i32, i32), d: (i32, i32), region: Region) {
        let w = region.max.0 - region.min.0 + 1;
        let h = region.max.1 - region.min.1 + 1;
        let out_dir = (-d.0, -d.1);
        let biome = biome_at(door.0, door.1, self.world.seed);
        let mut room = Room::build(w, h, region.min, door, out_dir, house_floor(biome));
        if let Some(old) = self.rooms.get(&door) {
            room.copy_from(old); // casa ampliada: guarda o que ja existia la dentro
        }
        let inward = (room.door_in.0 - room.out_dir.0, room.door_in.1 - room.out_dir.1);
        let text = format!("Casa {}x{}  ->  interior {}x{}", w, h, room.w, room.h);
        self.rooms.insert(door, room);
        self.dim = Dim::House(door);
        self.player.pos = vec2(inward.0 as f32 + 0.5, inward.1 as f32 + 0.5);
        self.cam = self.player.pos;
        self.last_cell = inward;
        self.breaking = None;
        self.particles.clear();
        self.fade = 1.0;
        self.say(&text, 2.5);
    }

    fn exit_house(&mut self, door_out: (i32, i32), out_dir: (i32, i32)) {
        let out = (door_out.0 + out_dir.0, door_out.1 + out_dir.1);
        self.dim = Dim::Over;
        self.player.pos = vec2(out.0 as f32 + 0.5, out.1 as f32 + 0.5);
        self.cam = self.player.pos;
        self.last_cell = out;
        self.breaking = None;
        self.particles.clear();
        self.fade = 1.0;
    }
}

// =====================================================================
// SECAO 10 - DESENHO
// =====================================================================
impl Game {
    fn hart(&self) -> f32 {
        self.settings.height3d.round().clamp(0.0, 10.0)
    }

    fn draw(&mut self, c: &Controls) {
        set_default_camera();
        let s = ui_scale();
        if matches!(self.dim, Dim::House(_)) {
            clear_background(BLACK); // dentro da casa o fundo e preto
        } else {
            clear_background(Color::new(0.04, 0.04, 0.06, 1.0));
        }

        if self.ui == UiMode::Gallery {
            set_camera(&ui_camera(s));
            self.begin_fit(1040.0, 740.0);
            self.draw_gallery(c);
            self.end_fit();
            if !self.touch_ui {
                self.draw_pointer_dot();
            }
            set_default_camera();
            return;
        }

        // ---- mundo (coordenadas reais da tela)
        self.draw_scene();
        self.draw_overlays();
        self.draw_particles();
        if self.ui == UiMode::None {
            self.draw_aim_cursor();
        }
        if self.player.hurt_t > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.8, 0.0, 0.0, 0.25 * (self.player.hurt_t / 0.6)),
            );
        }

        // ---- interface (tamanho virtual, ampliado para a tela)
        set_camera(&ui_camera(s));
        self.draw_hud();
        match self.ui {
            UiMode::Inventory | UiMode::Workbench => self.draw_container(),
            UiMode::Pause => self.menu_pause(c),
            UiMode::Settings => self.menu_settings(c),
            UiMode::Skin => self.menu_skin(c),
            _ => {}
        }
        if self.touch_ui {
            self.draw_touch_hud();
        } else {
            if self.settings.mobile_mode {
                self.draw_touch_hud();
            }
            if self.ui != UiMode::None || self.settings.mobile_mode {
                self.draw_pointer_dot();
            }
        }
        if self.show_controls {
            self.draw_controls_help();
        }
        set_default_camera();

        if self.fade > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, self.fade.min(1.0)),
            );
        }
    }

    /// Chao + piso (planos), depois TUDO que "esta em pe" ordenado por profundidade
    /// (blocos, arvores, drops e jogador) para dar a ideia de frente/atras.
    fn draw_scene(&self) {
        let ts = self.ts();
        let cam = self.cam_px();
        let half = self.half();
        let px = ts / TILE as f32;
        let x0 = ((cam.x - half.x) / ts).floor() as i32 - 1;
        let x1 = ((cam.x + half.x) / ts).ceil() as i32 + 1;
        let y0 = ((cam.y - half.y) / ts).floor() as i32 - 1;
        let y1 = ((cam.y + half.y) / ts).ceil() as i32 + 4; // arvores altas aparecem de baixo
        let in_room = matches!(self.dim, Dim::House(_));
        let mut list: Vec<(f32, DrawKind)> = Vec::with_capacity(512);

        for wy in y0..=y1 {
            for wx in x0..=x1 {
                let (f, cv, b) = self.g_cell(wx, wy);
                let sx = wx as f32 * ts - cam.x + half.x;
                let sy = wy as f32 * ts - cam.y + half.y;
                if !(in_room && f == Block::Void) {
                    draw_tile(&self.atlas, f as usize, sx, sy, ts);
                    if f == Block::Water && cv == Block::Air {
                        let ph = (self.time * 1.3 + hash01(wx, wy, 5) * 40.0).sin();
                        if ph > 0.8 {
                            let ox = (hash01(wx, wy, 6) * 10.0 + 2.0).floor();
                            let oy = (hash01(wx, wy, 7) * 10.0 + 2.0).floor();
                            draw_rectangle(sx + ox * px, sy + oy * px, 2.0 * px, px, Color::new(1.0, 1.0, 1.0, 0.5));
                        }
                    }
                    if cv != Block::Air {
                        // PISO: mais escuro e com borda, para diferenciar do chao
                        draw_tile_tint(&self.atlas, cv as usize, sx, sy, ts, Color::new(0.86, 0.86, 0.9, 1.0));
                        draw_rectangle_lines(sx, sy, ts, ts, 1.0, Color::new(0.0, 0.0, 0.0, 0.25));
                    }
                }
                if b != Block::Air {
                    let key = if b == Block::Door && self.door_near(wx, wy) {
                        wy as f32
                    } else if b.raised() || b.is_tree() {
                        wy as f32 + 0.95
                    } else {
                        wy as f32 + 0.5
                    };
                    list.push((key, DrawKind::Block(wx, wy, b)));
                }
            }
        }

        if self.dim == Dim::Over {
            for (i, ft) in self.falling.iter().enumerate() {
                list.push((ft.cell.1 as f32 + 0.9, DrawKind::Falling(i)));
            }
        }
        for (i, d) in self.drops.iter().enumerate() {
            if d.dim == self.dim {
                list.push((d.pos.y, DrawKind::Drop(i)));
            }
        }
        list.push((self.player.pos.y, DrawKind::Player));
        list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        for (_, k) in list.iter() {
            match *k {
                DrawKind::Block(wx, wy, b) => self.draw_block(wx, wy, b),
                DrawKind::Falling(i) => self.draw_falling(&self.falling[i]),
                DrawKind::Drop(i) => self.draw_drop(&self.drops[i]),
                DrawKind::Player => self.draw_player(),
            }
        }
    }

    fn draw_block(&self, wx: i32, wy: i32, b: Block) {
        let ts = self.ts();
        let s = self.cell_screen(wx, wy);
        if b.is_tree() {
            self.draw_tree_standing(s, b, wx, wy);
        } else if b == Block::Door && self.door_near(wx, wy) {
            // porta aberta: vao escuro rente ao chao (da para passar)
            draw_tile(&self.atlas, Block::DoorOpen as usize, s.x, s.y, ts);
        } else if b.raised() {
            self.draw_raised(b as usize, s.x, s.y);
        } else {
            draw_tile(&self.atlas, b as usize, s.x, s.y, ts);
        }
    }

    /// Bloco "em pe": o topo sobe alguns pixels e aparece a face da frente (mais escura).
    fn draw_raised(&self, idx: usize, sx: f32, sy: f32) {
        let ts = self.ts();
        let z = self.settings.zoom;
        let hart = self.hart();
        let hpx = hart * z;
        draw_tile(&self.atlas, idx, sx, sy - hpx, ts);
        if hart > 0.0 {
            let (ox, oy) = tile_origin(idx);
            let hh = hart as usize;
            draw_texture_ex(
                &self.atlas,
                sx,
                sy + ts - hpx,
                Color::new(0.6, 0.6, 0.68, 1.0),
                DrawTextureParams {
                    dest_size: Some(vec2(ts, hpx)),
                    source: Some(Rect::new(ox as f32, (oy + TILE - hh) as f32, TILE as f32, hh as f32)),
                    ..Default::default()
                },
            );
        }
    }

    fn draw_tree_standing(&self, s: Vec2, b: Block, wx: i32, wy: i32) {
        let ts = self.ts();
        let z = self.settings.zoom;
        let snowy = self.g_floor(wx, wy) == Block::Snow;
        let t = &self.trees[(if snowy { 3 } else { 0 }) + b.tree_size()];
        let w = t.w * z;
        let h = t.h * z;
        let cx = s.x + ts / 2.0;
        let base_y = s.y + ts - 2.0 * z;
        draw_ellipse(cx, base_y, w * 0.38, ts * 0.14, 0.0, Color::new(0.0, 0.0, 0.0, 0.25));
        draw_texture_ex(
            &t.tex,
            (cx - w / 2.0).round(),
            (base_y - h).round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
    }

    /// Arvore balancando, caindo para um lado e deitada no chao.
    fn draw_falling(&self, ft: &FallingTree) {
        let ts = self.ts();
        let z = self.settings.zoom;
        let t = &self.trees[(if ft.snowy { 3 } else { 0 }) + ft.size];
        let w = t.w * z;
        let h = t.h * z;
        let s = self.cell_screen(ft.cell.0, ft.cell.1);
        let bx = s.x + ts / 2.0;
        let by = s.y + ts - 2.0 * z;
        let fall_end = TREE_WARN + TREE_FALL;
        let angle = if ft.t < TREE_WARN {
            let u = ft.t / TREE_WARN;
            ft.dir * 0.06 * u + (ft.t * 45.0).sin() * 0.025 // avisa para qual lado vai cair
        } else if ft.t < fall_end {
            let u = (ft.t - TREE_WARN) / TREE_FALL;
            ft.dir * (0.06 + (PI / 2.0 - 0.06) * u * u)
        } else {
            ft.dir * PI / 2.0
        };
        let alpha = if ft.t > fall_end {
            (1.0 - (ft.t - fall_end) / TREE_LINGER).clamp(0.0, 1.0)
        } else {
            1.0
        };
        draw_ellipse(bx, by, w * 0.38, ts * 0.14, 0.0, Color::new(0.0, 0.0, 0.0, 0.25 * alpha));
        draw_texture_ex(
            &t.tex,
            bx - w / 2.0,
            by - h,
            Color::new(1.0, 1.0, 1.0, alpha),
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                rotation: angle * ROTATION_SIGN,
                pivot: Some(vec2(bx, by)),
                ..Default::default()
            },
        );
    }

    fn draw_overlays(&self) {
        let ts = self.ts();
        let z = self.settings.zoom;
        let hart = self.hart();
        // rachaduras do que esta quebrando
        if let Some(bs) = &self.breaking {
            let (_, _, b) = self.g_cell(bs.cell.0, bs.cell.1);
            let s = self.cell_screen(bs.cell.0, bs.cell.1);
            let lift = if bs.layer == Layer::Block && b.raised() { hart * z } else { 0.0 };
            let p = bs.progress.min(1.0);
            let sy = s.y - lift;
            draw_rectangle(s.x, sy, ts, ts, Color::new(0.0, 0.0, 0.0, 0.30 * p));
            let n = (p * 14.0) as usize;
            let px = ts / TILE as f32;
            for k in 0..n {
                let cx = ((k * 7 + 3) % 14 + 1) as f32;
                let cy = ((k * 11 + 5) % 14 + 1) as f32;
                draw_rectangle(s.x + cx * px, sy + cy * px, px, px, Color::new(0.05, 0.03, 0.02, 0.9));
            }
        }
        // contorno do bloco mirado
        if self.settings.show_target && self.ui == UiMode::None {
            let (_, _, b) = self.g_cell(self.target.0, self.target.1);
            let s = self.cell_screen(self.target.0, self.target.1);
            let col = Color::new(1.0, 1.0, 1.0, 0.5);
            if b.raised() {
                let lift = hart * z;
                draw_rectangle_lines(s.x, s.y - lift, ts, ts + lift, 2.0, col);
            } else {
                draw_rectangle_lines(s.x, s.y, ts, ts, 2.0, col);
            }
        }
    }

    fn draw_drop(&self, d: &ItemDrop) {
        let ts = self.ts();
        let size = (self.settings.zoom - 1.0).max(1.0) * TILE as f32;
        let s = self.to_screen(d.pos);
        let bob = (self.time * 3.0 + d.phase).sin() * 0.05 * ts;
        draw_ellipse(s.x, s.y + 0.15 * ts, size * 0.4, size * 0.15, 0.0, Color::new(0.0, 0.0, 0.0, 0.3));
        draw_tile(
            &self.atlas,
            d.item.icon_tile(),
            (s.x - size / 2.0).round(),
            (s.y - size / 2.0 + bob).round(),
            size,
        );
    }

    fn draw_particles(&self) {
        let ts = self.ts();
        for p in &self.particles {
            let s = self.to_screen(p.pos);
            let a = (p.life / p.max_life).clamp(0.0, 1.0);
            let size = (p.size * ts).max(2.0);
            draw_rectangle(
                s.x.round(),
                s.y.round(),
                size,
                size,
                Color::new(p.color.r, p.color.g, p.color.b, a),
            );
        }
    }

    fn draw_held(&self) {
        let Some(item) = self.held() else {
            return;
        };
        let dir = self.player.aim;
        let c = self.to_screen(self.player.pos + vec2(0.0, -0.3) + dir * 0.55);
        let size = TILE as f32 * (self.settings.zoom / 2.0).floor().max(1.0);
        let swing = if self.breaking.is_some() {
            (self.time * 14.0).sin() * 0.7
        } else if self.player.interact_t > 0.0 {
            self.player.interact_t * 2.0
        } else {
            0.0
        };
        let left = dir.x < 0.0;
        draw_texture_ex(
            &self.atlas,
            (c.x - size / 2.0).round(),
            (c.y - size / 2.0).round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                source: Some(tile_rect(item.icon_tile())),
                rotation: if left { -swing } else { swing },
                flip_x: left,
                ..Default::default()
            },
        );
    }

    fn draw_player(&self) {
        let ts = self.ts();
        let z = self.settings.zoom;
        let feet = self.to_screen(self.player.pos);
        draw_ellipse(
            feet.x,
            feet.y + 0.2 * ts,
            0.3 * ts,
            0.12 * ts,
            0.0,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );

        let facing_up = self.player.aim.y < -0.5;
        if facing_up {
            self.draw_held();
        }

        let frame = if self.breaking.is_some() {
            if ((self.time * 7.0) as i32) % 2 == 0 {
                3
            } else {
                0
            }
        } else if self.player.interact_t > 0.0 {
            3
        } else if self.player.walking {
            [1usize, 0, 2, 0][((self.player.anim_t * 8.0) as usize) % 4]
        } else {
            0
        };

        let hurt = self.player.hurt_t > 0.0;
        let blink_off = hurt && ((self.time * 20.0) as i32) % 2 == 0;
        if !blink_off {
            let tint = if hurt { Color::new(1.0, 0.6, 0.6, 1.0) } else { WHITE };
            let w = HUMAN_W as f32 * z;
            let h = HUMAN_H as f32 * z;
            let x = (feet.x - w / 2.0).round();
            let y = (feet.y + 0.28 * ts - h).round();
            self.human.draw(self.player.facing, frame, x, y, z, tint);
        }

        if !facing_up {
            self.draw_held();
        }
    }

    /// O "mouse" do jogo: ponto branco com borda preta orbitando o jogador.
    fn draw_aim_cursor(&self) {
        let p = self.to_screen(self.player.pos + self.player.aim * REACH);
        draw_circle(p.x, p.y, 2.2 * self.settings.zoom, BLACK);
        draw_circle(p.x, p.y, 1.4 * self.settings.zoom, WHITE);
    }

    /// Ponteiro normal (usado nos menus)
    fn draw_pointer_dot(&self) {
        draw_circle(self.ptr().x, self.ptr().y, 7.0, BLACK);
        draw_circle(self.ptr().x, self.ptr().y, 4.5, WHITE);
    }

    fn hotbar_origin(&self) -> Vec2 {
        let w = 9.0 * UI_SLOT + 8.0 * UI_GAP;
        vec2((self.ui_dims().0 - w) / 2.0, self.ui_dims().1 - UI_SLOT - 14.0)
    }

    fn draw_slot(&self, x: f32, y: f32, stack: Option<ItemStack>, selected: bool, hover: bool) {
        draw_rectangle(x, y, UI_SLOT, UI_SLOT, Color::new(0.08, 0.08, 0.1, 0.8));
        let border = if selected {
            WHITE
        } else if hover {
            Color::new(1.0, 1.0, 1.0, 0.6)
        } else {
            Color::new(0.35, 0.35, 0.4, 1.0)
        };
        draw_rectangle_lines(x, y, UI_SLOT, UI_SLOT, if selected { 4.0 } else { 2.0 }, border);
        if let Some(st) = stack {
            let icon = 48.0;
            draw_tile(
                &self.atlas,
                st.item.icon_tile(),
                x + (UI_SLOT - icon) / 2.0,
                y + (UI_SLOT - icon) / 2.0,
                icon,
            );
            if st.count > 1 {
                let t = format!("{}", st.count);
                let w = measure_text(&t, None, 22, 1.0).width;
                text_shadow(&t, x + UI_SLOT - w - 4.0, y + UI_SLOT - 6.0, 22.0, WHITE);
            }
        }
    }

    fn draw_hud(&self) {
        let hb = self.hotbar_origin();

        // vida: 20 pontos = 10 coracoes
        for i in 0..(MAX_HP / 2) {
            let hp = self.player.hp;
            let tile = if hp >= 2 * (i + 1) {
                T_HEART_FULL
            } else if hp == 2 * i + 1 {
                T_HEART_HALF
            } else {
                T_HEART_EMPTY
            };
            draw_heart(&self.atlas, tile, hb.x + i as f32 * 28.0, hb.y - 34.0);
        }

        let in_container = matches!(self.ui, UiMode::Inventory | UiMode::Workbench);
        if !in_container {
            for i in 0..9 {
                let x = hb.x + i as f32 * (UI_SLOT + UI_GAP);
                self.draw_slot(x, hb.y, self.inv.slots[i], i == self.selected, false);
            }
            if let Some(st) = self.inv.slots[self.selected] {
                text_centered(&st.item.name(), self.ui_dims().0 / 2.0, hb.y - 44.0, 24.0, WHITE);
            }
        }

        if self.settings.show_help && self.help_t > 0.0 && self.ui == UiMode::None {
            let a = (self.help_t / 3.0).clamp(0.0, 1.0);
            let col = Color::new(1.0, 1.0, 1.0, a);
            let lines = [
                "WASD/Setas: andar    Mouse: mirar    Esq (segure): quebrar    Dir: colocar/usar    Shift+Dir: colocar como piso",
                "E: inventario e crafting    1-9/Roda: slot    Q: soltar    +/-: zoom    Esc: menu    F1: galeria    F3: debug    H: controles",
            ];
            let touch_lines = [
                "Esquerda: joystick (andar)    BATER/USAR: arraste no botao para mirar    Toque no mundo: mirar",
                "Toque na hotbar: escolher item    Inv: inventario e crafting    Piso: liga/desliga    Menu: pausa    ?: controles",
            ];
            let mobile_lines = [
                "Botoes na tela = teclas (Esc, E, Q, F1...)    Joystick ou WASD: andar    BATER / USAR: clique no botao",
                "A mira e automatica: o bloco alvo fica na direcao em que voce anda    H: lista de controles",
            ];
            let lines: [&str; 2] = if self.touch_ui {
                touch_lines
            } else if self.settings.mobile_mode {
                mobile_lines
            } else {
                lines
            };
            for (i, l) in lines.iter().enumerate() {
                text_shadow(l, 16.0, 28.0 + i as f32 * 24.0, 22.0, col);
            }
        }

        if let Some((m, _)) = &self.msg {
            text_centered(m, self.ui_dims().0 / 2.0, 90.0, 28.0, Color::new(1.0, 0.9, 0.5, 1.0));
        }

        if self.settings.show_fps {
            let t = format!("{} FPS", get_fps());
            text_shadow(&t, self.ui_dims().0 - 100.0, 24.0, 22.0, Color::new(0.7, 1.0, 0.7, 1.0));
        }

        if self.show_debug {
            let p = self.player.pos;
            let biome = biome_at(p.x.floor() as i32, p.y.floor() as i32, self.world.seed);
            let lines = [
                format!("FPS: {}", get_fps()),
                format!("pos: {:.2}, {:.2}   {:?}", p.x, p.y, self.dim),
                format!("bioma: {}", biome.name()),
                format!("chunks: {}  mudas: {}  arvores caindo: {}", self.world.chunks.len(), self.world.saplings.len(), self.falling.len()),
                format!("seed: {}", self.world.seed),
                format!(
                    "alvo: {:?}  (chao, piso, bloco) = {:?}",
                    self.target,
                    self.g_cell(self.target.0, self.target.1)
                ),
            ];
            for (i, l) in lines.iter().enumerate() {
                text_shadow(l, 16.0, 120.0 + i as f32 * 22.0, 20.0, Color::new(0.7, 1.0, 0.7, 1.0));
            }
        }
    }

    /// Inventario (E) com crafting 2x2, ou bancada (clique direito nela) com crafting 3x3.
    fn draw_container(&self) {
        draw_rectangle(0.0, 0.0, self.ui_dims().0, self.ui_dims().1, Color::new(0.0, 0.0, 0.0, 0.55));
        let panel = self.container_panel();
        draw_rectangle(panel.x, panel.y, panel.w, panel.h, Color::new(0.16, 0.15, 0.2, 0.97));
        draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 3.0, Color::new(0.5, 0.5, 0.6, 1.0));
        let title = if self.ui == UiMode::Workbench {
            "Bancada  (grade 3x3)"
        } else {
            "Inventario  (grade 2x2)"
        };
        text_shadow(title, panel.x + UI_PAD, panel.y + 32.0, 28.0, WHITE);

        let mut hover_name: Option<String> = None;

        // grade de crafting
        for i in 0..self.craft_n * self.craft_n {
            let r = self.craft_slot_rect(i);
            let hover = r.contains(self.ptr());
            self.draw_slot(r.x, r.y, self.craft[i], false, hover);
            if hover {
                if let Some(st) = self.craft[i] {
                    hover_name = Some(st.item.name());
                }
            }
        }
        // seta + resultado
        let o = self.craft_origin();
        let step = UI_SLOT + UI_GAP;
        let n = self.craft_n as f32;
        let ay = o.y + (n * step - UI_GAP) / 2.0;
        let ax = o.x + n * step + 14.0;
        draw_rectangle(ax, ay - 4.0, 28.0, 8.0, Color::new(0.8, 0.8, 0.85, 1.0));
        draw_triangle(
            vec2(ax + 28.0, ay - 14.0),
            vec2(ax + 28.0, ay + 14.0),
            vec2(ax + 46.0, ay),
            Color::new(0.8, 0.8, 0.85, 1.0),
        );
        let ro = self.craft_out_rect();
        let out_stack = self.craft_out.and_then(|(it, k)| make_stack(it, k));
        self.draw_slot(ro.x, ro.y, out_stack, out_stack.is_some(), ro.contains(self.ptr()));
        if ro.contains(self.ptr()) {
            if let Some(st) = out_stack {
                hover_name = Some(st.item.name());
            }
        }

        // inventario
        for idx in 0..36 {
            let r = self.inv_slot_rect(idx);
            let hover = r.contains(self.ptr());
            self.draw_slot(r.x, r.y, self.inv.slots[idx], idx == self.selected, hover);
            if hover {
                if let Some(st) = self.inv.slots[idx] {
                    hover_name = Some(st.item.name());
                }
            }
        }
        if let Some(nm) = hover_name {
            text_centered(&nm, panel.x + panel.w / 2.0, panel.y + panel.h + 30.0, 24.0, WHITE);
        }

        // item "na mao" do cursor
        if let Some(st) = self.cursor_stack {
            let icon = 48.0;
            draw_tile(
                &self.atlas,
                st.item.icon_tile(),
                self.ptr().x - icon / 2.0,
                self.ptr().y - icon / 2.0,
                icon,
            );
            if st.count > 1 {
                let t = format!("{}", st.count);
                text_shadow(&t, self.ptr().x + 6.0, self.ptr().y + 26.0, 22.0, WHITE);
            }
        }
    }
}

// =====================================================================
// SECAO 11 - MENUS (pausa, configuracoes, personagem) E GALERIA
//  Menus em "modo imediato": cada widget desenha e ja devolve o clique.
// =====================================================================

const SWATCH_HEX: [u32; 24] = [
    // tons de pele
    0xffe0c4, 0xf2b890, 0xd99468, 0xb97a4e, 0x8d5a36, 0x5c3a21,
    // outras cores
    0x1a1a1a, 0x4b2e1b, 0x9c6b3a, 0xd8b45a, 0xe8d8a0, 0xc2452d,
    0xf4f4f4, 0x8a8a92, 0x3a6ad8, 0x3aa8bd, 0x3ab86a, 0x86cf5a,
    0xf2d33c, 0xe8913a, 0xe05a9a, 0x8a3ad8, 0x2b3680, 0x6e1f2e,
];

fn swatch_rgb(i: usize) -> [u8; 3] {
    let v = SWATCH_HEX[i];
    [(v >> 16) as u8, (v >> 8) as u8, v as u8]
}

impl Game {
    fn ui_button(&self, c: &Controls, r: Rect, label: &str, active: bool) -> bool {
        let hover = r.contains(self.ptr());
        let base = if active {
            Color::new(0.28, 0.5, 0.34, 1.0)
        } else if hover {
            Color::new(0.35, 0.35, 0.5, 1.0)
        } else {
            Color::new(0.2, 0.2, 0.28, 1.0)
        };
        draw_rectangle(r.x, r.y, r.w, r.h, base);
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 2.0, Color::new(0.6, 0.6, 0.7, 1.0));
        text_centered(label, r.x + r.w / 2.0, r.y + r.h / 2.0 + 7.0, 24.0, WHITE);
        hover && c.attack_pressed
    }

    /// Botao liga/desliga. Devolve o novo valor.
    fn ui_toggle(&self, c: &Controls, r: Rect, label: &str, value: bool) -> bool {
        let text = format!("{}: {}", label, if value { "Sim" } else { "Nao" });
        if self.ui_button(c, r, &text, value) {
            !value
        } else {
            value
        }
    }

    /// Barra deslizante. `r` = area da linha toda (50 de altura). Devolve o novo valor.
    fn ui_slider(&mut self, c: &Controls, id: u32, r: Rect, label: &str, value: f32, min: f32, max: f32, shown: &str) -> f32 {
        let hit = Rect::new(r.x - 8.0, r.y + 22.0, r.w + 16.0, 26.0).contains(self.ptr());
        if c.attack_pressed && hit {
            self.ui_active = Some(id);
        }
        let mut v = value;
        if self.ui_active == Some(id) {
            if c.attack_down {
                let t = ((self.ptr().x - r.x) / r.w).clamp(0.0, 1.0);
                v = min + t * (max - min);
            } else {
                self.ui_active = None;
            }
        }
        let t = ((v - min) / (max - min)).clamp(0.0, 1.0);
        text_shadow(&format!("{}: {}", label, shown), r.x, r.y + 18.0, 22.0, WHITE);
        draw_rectangle(r.x, r.y + 30.0, r.w, 8.0, Color::new(0.25, 0.25, 0.32, 1.0));
        draw_rectangle(r.x, r.y + 30.0, r.w * t, 8.0, Color::new(0.4, 0.7, 0.5, 1.0));
        draw_rectangle(r.x + r.w * t - 6.0, r.y + 24.0, 12.0, 20.0, WHITE);
        v
    }

    fn dim_background(&self) {
        draw_rectangle(0.0, 0.0, self.ui_dims().0, self.ui_dims().1, Color::new(0.0, 0.0, 0.0, 0.65));
    }

    // ----- menu de pausa (Esc) -----
    fn menu_pause(&mut self, c: &Controls) {
        self.ui_active = None;
        self.dim_background();
        let cx = self.ui_dims().0 / 2.0;
        let cy = self.ui_dims().1 / 2.0;
        text_centered("BIO  -  Pausado", cx, cy - 170.0, 40.0, WHITE);
        let w = 340.0;
        let labels = ["Continuar", "Personagem (skin)", "Configuracoes", "Galeria de sprites", "Sair do jogo"];
        for (i, l) in labels.iter().enumerate() {
            let r = Rect::new(cx - w / 2.0, cy - 120.0 + i as f32 * 62.0, w, 48.0);
            if self.ui_button(c, r, l, false) {
                match i {
                    0 => {
                        self.ui = UiMode::None;
                        self.wait_release = true;
                    }
                    1 => {
                        self.skin_list = list_skins();
                        self.ui = UiMode::Skin;
                    }
                    2 => self.ui = UiMode::Settings,
                    3 => self.ui = UiMode::Gallery,
                    _ => {
                        save_config(&self.settings, &self.skin);
                        std::process::exit(0);
                    }
                }
            }
        }
    }

    // ----- configuracoes -----
    fn menu_settings(&mut self, c: &Controls) {
        self.dim_background();
        let pw = 780.0;
        let ph = 520.0;
        let panel = Rect::new((self.ui_dims().0 - pw) / 2.0, (self.ui_dims().1 - ph) / 2.0, pw, ph);
        draw_rectangle(panel.x, panel.y, panel.w, panel.h, Color::new(0.16, 0.15, 0.2, 0.98));
        draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 3.0, Color::new(0.5, 0.5, 0.6, 1.0));
        text_centered("Configuracoes", panel.x + pw / 2.0, panel.y + 44.0, 36.0, WHITE);

        // coluna esquerda: barras
        let sx = panel.x + 30.0;
        let sy = panel.y + 90.0;
        let z = self.settings.zoom;
        let nz = self
            .ui_slider(c, 1, Rect::new(sx, sy, 320.0, 50.0), "Zoom", z, 2.0, 10.0, &format!("{}x", z.round() as i32))
            .round();
        self.settings.zoom = nz;

        let h3 = self.settings.height3d;
        let nh = self
            .ui_slider(
                c,
                2,
                Rect::new(sx, sy + 70.0, 320.0, 50.0),
                "Altura do efeito 3D",
                h3,
                0.0,
                10.0,
                &format!("{} px", h3.round() as i32),
            )
            .round();
        self.settings.height3d = nh;

        let cs = self.settings.cam_smooth;
        let ncs = self.ui_slider(
            c,
            3,
            Rect::new(sx, sy + 140.0, 320.0, 50.0),
            "Suavidade da camera",
            cs,
            4.0,
            30.0,
            &format!("{}", cs.round() as i32),
        );
        self.settings.cam_smooth = ncs;

        // coluna direita: liga/desliga
        let tx = panel.x + 400.0;
        let ty = panel.y + 92.0;
        let w = 350.0;
        let h = 44.0;
        let v = self.settings.show_target;
        self.settings.show_target = self.ui_toggle(c, Rect::new(tx, ty, w, h), "Contorno do alvo", v);
        let v = self.settings.show_fps;
        self.settings.show_fps = self.ui_toggle(c, Rect::new(tx, ty + 54.0, w, h), "Mostrar FPS", v);
        let v = self.settings.show_help;
        self.settings.show_help = self.ui_toggle(c, Rect::new(tx, ty + 108.0, w, h), "Mostrar ajuda", v);
        let v = self.settings.particles;
        self.settings.particles = self.ui_toggle(c, Rect::new(tx, ty + 162.0, w, h), "Particulas", v);
        let v = self.settings.tree_hurts;
        self.settings.tree_hurts = self.ui_toggle(c, Rect::new(tx, ty + 216.0, w, h), "Arvores machucam", v);
        let v = self.settings.mobile_mode;
        self.settings.mobile_mode = self.ui_toggle(c, Rect::new(tx, ty + 270.0, w, h), "Modo celular (teste)", v);

        text_shadow(
            "Salvo automaticamente em bio_config.txt ao voltar.",
            panel.x + 30.0,
            panel.y + ph - 80.0,
            20.0,
            Color::new(0.75, 0.75, 0.8, 1.0),
        );
        if self.ui_button(c, Rect::new(panel.x + pw / 2.0 - 110.0, panel.y + ph - 62.0, 220.0, 46.0), "Voltar", false) {
            self.leave_submenu();
        }
    }

    /// Recria a textura do jogador com a skin atual.
    fn apply_skin(&mut self) {
        let img = build_human_image(&self.skin);
        self.human.tex.update(&img);
        self.skin_dirty = true;
    }

    // ----- personagem: genero, cores, nome, exportar e importar skin -----
    /// Encaixa o painel grande na tela (importante no celular) e desenha o menu.
    fn menu_skin(&mut self, c: &Controls) {
        self.begin_fit(960.0, 640.0);
        self.menu_skin_inner(c);
        self.end_fit();
    }

    fn menu_skin_inner(&mut self, c: &Controls) {
        self.dim_background();
        let pw = 960.0;
        let ph = 640.0;
        let panel = Rect::new((self.ui_dims().0 - pw) / 2.0, (self.ui_dims().1 - ph) / 2.0, pw, ph);
        draw_rectangle(panel.x, panel.y, panel.w, panel.h, Color::new(0.16, 0.15, 0.2, 0.98));
        draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 3.0, Color::new(0.5, 0.5, 0.6, 1.0));
        text_shadow("Personagem", panel.x + 30.0, panel.y + 44.0, 36.0, WHITE);

        let mut changed = false;

        // ---- previa (gira pelas 8 direcoes andando)
        let pv = Rect::new(panel.x + 30.0, panel.y + 70.0, 270.0, 330.0);
        draw_rectangle(pv.x, pv.y, pv.w, pv.h, Color::new(0.1, 0.1, 0.14, 1.0));
        draw_rectangle_lines(pv.x, pv.y, pv.w, pv.h, 2.0, Color::new(0.4, 0.4, 0.5, 1.0));
        let dir = ((self.time * 0.7) as usize) % 8;
        let fr = [1usize, 0, 2, 0][((self.time * 8.0) as usize) % 4];
        self.human.draw(
            dir,
            fr,
            pv.x + (pv.w - HUMAN_W as f32 * 12.0) / 2.0,
            pv.y + 60.0,
            12.0,
            WHITE,
        );
        text_centered(&self.skin.name.clone(), pv.x + pv.w / 2.0, pv.y + pv.h - 14.0, 24.0, WHITE);

        // ---- genero
        let rx = panel.x + 330.0;
        let mut y = panel.y + 76.0;
        let female = self.skin.female;
        if self.ui_button(c, Rect::new(rx, y, 150.0, 40.0), "Masculino", !female) && female {
            self.skin.female = false;
            changed = true;
        }
        if self.ui_button(c, Rect::new(rx + 160.0, y, 150.0, 40.0), "Feminino", female) && !female {
            self.skin.female = true;
            changed = true;
        }

        // ---- abas das partes
        y += 56.0;
        for i in 0..6usize {
            let r = Rect::new(rx + i as f32 * 98.0, y, 92.0, 38.0);
            if self.ui_button(c, r, SKIN_PART_NAMES[i], self.skin_part == i) {
                self.skin_part = i;
            }
        }

        // ---- amostras de cor
        y += 54.0;
        let part = self.skin_part;
        let cur = self.skin.part(part);
        for k in 0..24usize {
            let col = swatch_rgb(k);
            let r = Rect::new(rx + (k % 12) as f32 * 42.0, y + (k / 12) as f32 * 42.0, 36.0, 36.0);
            draw_rectangle(r.x, r.y, r.w, r.h, rgb_color(col));
            let selected = col == cur;
            draw_rectangle_lines(
                r.x,
                r.y,
                r.w,
                r.h,
                if selected { 4.0 } else { 1.0 },
                if selected { WHITE } else { Color::new(0.0, 0.0, 0.0, 0.6) },
            );
            if r.contains(self.ptr()) && c.attack_pressed {
                self.skin.set_part(part, col);
                changed = true;
            }
        }

        // ---- barras RGB + previa da cor
        y += 96.0;
        let names = ["Vermelho", "Verde", "Azul"];
        for ch in 0..3usize {
            let cur = self.skin.part(part);
            let v = self.ui_slider(
                c,
                10 + ch as u32,
                Rect::new(rx, y + ch as f32 * 54.0, 360.0, 50.0),
                names[ch],
                cur[ch] as f32,
                0.0,
                255.0,
                &format!("{}", cur[ch]),
            );
            let nv = v.round() as u8;
            if nv != cur[ch] {
                let mut cc = self.skin.part(part);
                cc[ch] = nv;
                self.skin.set_part(part, cc);
                changed = true;
            }
        }
        let cur = self.skin.part(part);
        draw_rectangle(rx + 390.0, y + 10.0, 110.0, 110.0, rgb_color(cur));
        draw_rectangle_lines(rx + 390.0, y + 10.0, 110.0, 110.0, 3.0, WHITE);
        text_shadow(
            &format!("#{}", rgb_hex(cur)),
            rx + 390.0,
            y + 146.0,
            22.0,
            Color::new(0.85, 0.85, 0.9, 1.0),
        );

        // ---- nome + exportar
        y += 172.0;
        let name_r = Rect::new(rx, y, 300.0, 42.0);
        draw_rectangle(name_r.x, name_r.y, name_r.w, name_r.h, Color::new(0.08, 0.08, 0.1, 1.0));
        draw_rectangle_lines(
            name_r.x,
            name_r.y,
            name_r.w,
            name_r.h,
            if self.name_edit { 3.0 } else { 1.0 },
            if self.name_edit { WHITE } else { Color::new(0.5, 0.5, 0.6, 1.0) },
        );
        if c.attack_pressed {
            self.name_edit = !self.touch_ui && name_r.contains(self.ptr());
        }
        if self.name_edit {
            for ch in c.typed.iter() {
                if (ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '-') && self.skin.name.chars().count() < 16 {
                    self.skin.name.push(*ch);
                }
            }
            if c.backspace {
                self.skin.name.pop();
            }
        }
        let blink = self.name_edit && ((self.time * 2.0) as i32) % 2 == 0;
        let shown = format!("{}{}", self.skin.name, if blink { "_" } else { "" });
        text_shadow(&shown, name_r.x + 10.0, name_r.y + 29.0, 26.0, WHITE);
        let nm_hint = if self.touch_ui {
            "No celular o nome e automatico (skin1, skin2...)"
        } else {
            "Nome da skin (clique para digitar)"
        };
        text_shadow(nm_hint, name_r.x, name_r.y - 8.0, 18.0, Color::new(0.75, 0.75, 0.8, 1.0));

        if self.ui_button(c, Rect::new(rx + 315.0, y, 180.0, 42.0), "Exportar skin", false) {
            if self.touch_ui {
                self.skin.name = format!("skin{}", self.skin_list.len() + 1);
            }
            match export_skin(&self.skin) {
                Ok(path) => {
                    self.skin.name = clean_name(&self.skin.name);
                    self.skin_list = list_skins();
                    let t = format!("Skin exportada: {}", path);
                    self.say(&t, 4.0);
                }
                Err(e) => {
                    let t = format!("Erro ao exportar: {}", e);
                    self.say(&t, 4.0);
                }
            }
        }

        // ---- importar (lista a pasta skins/)
        let ix = panel.x + 30.0;
        let iy = panel.y + 430.0;
        text_shadow(
            "Importar skin  (arquivos .bioskin da pasta skins/ ao lado do jogo)",
            ix,
            iy,
            20.0,
            Color::new(0.85, 0.85, 0.9, 1.0),
        );
        let names_list = self.skin_list.clone();
        if names_list.is_empty() {
            text_shadow(
                "Nenhuma skin ainda. Exporte uma, ou coloque arquivos .bioskin na pasta skins/.",
                ix,
                iy + 34.0,
                20.0,
                Color::new(0.65, 0.65, 0.7, 1.0),
            );
        }
        for (i, nm) in names_list.iter().take(12).enumerate() {
            let r = Rect::new(ix + (i % 4) as f32 * 222.0, iy + 14.0 + (i / 4) as f32 * 46.0, 212.0, 38.0);
            if self.ui_button(c, r, nm, false) {
                if let Some(s) = import_skin(nm) {
                    self.skin = s;
                    changed = true;
                    let t = format!("Skin importada: {}", nm);
                    self.say(&t, 3.0);
                } else {
                    self.say("Arquivo de skin invalido", 3.0);
                }
            }
        }

        // ---- rodape
        if self.ui_button(c, Rect::new(panel.x + pw - 380.0, panel.y + ph - 58.0, 170.0, 42.0), "Padrao", false) {
            self.skin = Skin::standard();
            changed = true;
        }
        if self.ui_button(c, Rect::new(panel.x + pw - 200.0, panel.y + ph - 58.0, 170.0, 42.0), "Voltar", false) {
            if changed {
                self.apply_skin();
            }
            self.leave_submenu();
            return;
        }
        if let Some((m, _)) = &self.msg {
            text_shadow(m, panel.x + 30.0, panel.y + ph - 22.0, 22.0, Color::new(1.0, 0.9, 0.5, 1.0));
        }
        if changed {
            self.apply_skin();
        }
    }

    /// F1 / menu: mostra todos os sprites ampliados para voce conferir suas edicoes.
    fn draw_gallery(&mut self, c: &Controls) {
        text_shadow("GALERIA DE SPRITES   (F1 ou Esc fecha)", 20.0, 34.0, 30.0, WHITE);
        let gw = self.ui_dims().0;
        if self.ui_button(c, Rect::new(gw - 170.0, 12.0, 150.0, 42.0), "Voltar", false) {
            self.leave_submenu();
            return;
        }
        text_shadow(
            "Edite a SECAO 1 do src/main.rs, salve, rode de novo e veja aqui.",
            20.0,
            58.0,
            20.0,
            Color::new(0.8, 0.8, 0.85, 1.0),
        );

        // blocos, ferramentas, itens e coracoes (numero = indice no atlas)
        let size = 48.0;
        for idx in 1..=T_HEART_EMPTY {
            let col = (idx - 1) % 16;
            let row = (idx - 1) / 16;
            let x = 24.0 + col as f32 * 62.0;
            let y = 84.0 + row as f32 * 66.0;
            draw_rectangle(x - 3.0, y - 3.0, size + 6.0, size + 6.0, Color::new(0.2, 0.2, 0.25, 1.0));
            if idx >= T_HEART_FULL {
                draw_heart(&self.atlas, idx, x, y + 10.0);
            } else {
                draw_tile(&self.atlas, idx, x, y, size);
            }
            text_shadow(&format!("{}", idx), x, y + size + 12.0, 14.0, Color::new(0.8, 0.8, 0.8, 1.0));
        }

        // arvores (normal e neve)
        let mut tx = 24.0;
        for t in self.trees.iter() {
            let s = 2.0;
            draw_texture_ex(
                &t.tex,
                tx,
                300.0 - t.h * s + 60.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(t.w * s, t.h * s)),
                    ..Default::default()
                },
            );
            tx += t.w * s + 14.0;
        }
        text_shadow("Arvores: pequena, media, grande (normal e na neve)", 24.0, 372.0, 16.0, Color::new(0.7, 0.7, 0.75, 1.0));

        // as 5 vistas desenhadas x 4 quadros
        let names = ["Frente (S)", "Diag. frente (SE)", "Lado (E)", "Diag. costas (NE)", "Costas (N)"];
        let sc = 4.0;
        let base_y = 396.0;
        for v in 0..5usize {
            let x0 = 24.0 + v as f32 * 244.0;
            for f in 0..4usize {
                let x = x0 + f as f32 * (HUMAN_W as f32 * sc + 4.0);
                self.human.draw_raw(v, f, false, x, base_y, sc, WHITE);
            }
            text_shadow(names[v], x0, base_y + HUMAN_H as f32 * sc + 16.0, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
        }

        // as 8 direcoes de verdade: andando e interagindo
        let dirs = ["L", "SE", "S", "SO", "O", "NO", "N", "NE"];
        let fr = [1usize, 0, 2, 0][((self.time * 8.0) as usize) % 4];
        let y1 = base_y + 100.0;
        let y2 = y1 + 86.0;
        for d in 0..8usize {
            let x = 24.0 + d as f32 * 80.0;
            self.human.draw(d, fr, x, y1, sc, WHITE);
            self.human.draw(d, 3, x, y2, sc, WHITE);
            text_shadow(dirs[d], x + 12.0, y2 + 82.0, 18.0, WHITE);
        }
        text_shadow("andando", 700.0, y1 + 36.0, 20.0, Color::new(0.8, 0.8, 0.85, 1.0));
        text_shadow("interagindo", 700.0, y2 + 36.0, 20.0, Color::new(0.8, 0.8, 0.85, 1.0));
    }
}

fn rgb_color(c: [u8; 3]) -> Color {
    Color::from_rgba(c[0], c[1], c[2], 255)
}

// =====================================================================
// SECAO 12 - MAIN
// =====================================================================

fn window_conf() -> Conf {
    Conf {
        window_title: "Bio".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Primeiro quadro: tela de carregamento. Assim, enquanto o mundo e gerado
    // (pode levar 1s num celular), aparece algo em vez de tela preta.
    clear_background(Color::new(0.04, 0.04, 0.06, 1.0));
    draw_text("Carregando Bio...", 40.0, 90.0, 48.0, WHITE);
    next_frame().await;

    // O cursor do sistema some (o jogo desenha o proprio). No Android nao existe cursor.
    #[cfg(not(target_os = "android"))]
    show_mouse(false);

    let mut game = Game::new();
    loop {
        let dt = get_frame_time().min(0.05);
        let controls = if game.touch_ui {
            game.read_touch(dt)
        } else if game.settings.mobile_mode {
            game.read_mobile_pc(dt)
        } else {
            read_controls()
        };
        game.update(&controls, dt);
        game.draw(&controls);
        next_frame().await;
    }
}

// =====================================================================
// SECAO 13 - CELULAR (ANDROID): escala da interface e CONTROLES DE TOQUE
//
//  * A interface e desenhada num tamanho "virtual" fixo e ampliada para a
//    tela (ui_scale), entao funciona em qualquer resolucao de celular.
//  * Os controles de toque preenchem a MESMA struct `Controls` do teclado:
//    o resto do jogo nao sabe se voce esta no PC ou no celular.
//
//  Como jogar no celular (modo paisagem):
//    - Joystick (lado esquerdo): toque e arraste = andar nas 8 direcoes.
//    - BATER (segure) / USAR: arraste o dedo em cima do botao para MIRAR
//      (o ponto branco gira em volta do jogador, como o mouse).
//    - Tocar no mundo (lado direito) tambem mira naquela direcao.
//    - Toque nos slots da hotbar para escolher o item.
//    - Botoes no topo: Menu, Inv, F4, F3, F1, Z+, Z-, Q e Piso (liga/desliga).
//    - Inventario/bancada: toque = clique esquerdo, segurar = clique direito.
// =====================================================================

/// Escala da interface: tela real / tamanho virtual.
fn ui_scale() -> f32 {
    let base = if cfg!(target_os = "android") { 600.0 } else { 720.0 };
    (screen_width().min(screen_height()) / base).max(0.4)
}

/// Camera que faz as coordenadas da interface serem "virtuais" (origem no canto
/// superior esquerdo, y para baixo).
fn ui_camera(scale: f32) -> Camera2D {
    let w = screen_width() / scale;
    let h = screen_height() / scale;
    Camera2D {
        target: vec2(w / 2.0, h / 2.0),
        zoom: vec2(2.0 / w, -2.0 / h),
        ..Default::default()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TouchBtn {
    Floor,
    Drop,
    ZoomOut,
    ZoomIn,
    Gallery,
    Debug,
    Hurt,
    Inv,
    Menu,
    Help,
}

fn touch_label(b: TouchBtn) -> &'static str {
    match b {
        TouchBtn::Floor => "Piso",
        TouchBtn::Drop => "Q",
        TouchBtn::ZoomOut => "Z-",
        TouchBtn::ZoomIn => "Z+",
        TouchBtn::Gallery => "F1",
        TouchBtn::Debug => "F3",
        TouchBtn::Hurt => "F4",
        TouchBtn::Inv => "Inv",
        TouchBtn::Menu => "Menu",
        TouchBtn::Help => "?",
    }
}

struct TouchState {
    joy_id: Option<u64>,
    joy_origin: Vec2, // em coordenadas virtuais
    joy_pos: Vec2,
    atk_id: Option<u64>,
    use_id: Option<u64>,
    aim_id: Option<u64>, // dedo no mundo, so para mirar
    ptr_id: Option<u64>, // dedo que faz o papel de "mouse" nos menus
    ptr_t: f32,
    ptr_long: bool,
    last_ptr: Vec2, // em pixels reais
    aim: Vec2,
    floor_latched: bool,
}

impl TouchState {
    fn new() -> TouchState {
        TouchState {
            joy_id: None,
            joy_origin: Vec2::ZERO,
            joy_pos: Vec2::ZERO,
            atk_id: None,
            use_id: None,
            aim_id: None,
            ptr_id: None,
            ptr_t: 0.0,
            ptr_long: false,
            last_ptr: Vec2::ZERO,
            aim: vec2(0.0, 1.0),
            floor_latched: false,
        }
    }
}

impl Controls {
    fn none() -> Controls {
        Controls {
            move_axis: Vec2::ZERO,
            pointer: Vec2::ZERO,
            attack_down: false,
            attack_pressed: false,
            use_down: false,
            use_pressed: false,
            place_floor: false,
            slot_select: None,
            scroll: 0,
            toggle_inventory: false,
            cancel: false,
            drop_item: false,
            zoom_delta: 0,
            toggle_debug: false,
            toggle_gallery: false,
            test_hurt: false,
            toggle_help: false,
            typed: Vec::new(),
            backspace: false,
            aim_override: None,
        }
    }
}

impl Game {
    /// Tamanho da interface em unidades virtuais (ja considera o "encaixe" dos menus grandes).
    fn ui_dims(&self) -> (f32, f32) {
        let s = ui_scale() * self.fit_k;
        (screen_width() / s, screen_height() / s)
    }

    /// Posicao do ponteiro (mouse ou dedo) em unidades virtuais da interface.
    fn ptr(&self) -> Vec2 {
        self.pointer / (ui_scale() * self.fit_k)
    }

    /// Encolhe a interface para caber um painel grande de `pw` x `ph` (menus no celular).
    fn begin_fit(&mut self, pw: f32, ph: f32) {
        self.fit_k = 1.0;
        let (bw, bh) = self.ui_dims();
        let k = ((bh - 12.0) / ph).min((bw - 12.0) / pw).clamp(0.3, 1.0);
        self.fit_k = k;
        set_camera(&ui_camera(ui_scale() * k));
    }

    fn end_fit(&mut self) {
        self.fit_k = 1.0;
        set_camera(&ui_camera(ui_scale()));
    }

    // ----- layout dos controles de toque (coordenadas virtuais) -----
    fn touch_buttons(&self) -> Vec<(Rect, TouchBtn)> {
        let (uw, _) = self.ui_dims();
        let order = [
            TouchBtn::Menu,
            TouchBtn::Inv,
            TouchBtn::Hurt,
            TouchBtn::Debug,
            TouchBtn::Gallery,
            TouchBtn::ZoomIn,
            TouchBtn::ZoomOut,
            TouchBtn::Drop,
            TouchBtn::Floor,
            TouchBtn::Help,
        ];
        let mut v = Vec::new();
        for (i, b) in order.iter().enumerate() {
            v.push((Rect::new(uw - (i as f32 + 1.0) * 72.0 - 4.0, 10.0, 66.0, 46.0), *b));
        }
        v
    }

    /// ((centro, raio) do BATER, (centro, raio) do USAR)
    fn touch_action_circles(&self) -> ((Vec2, f32), (Vec2, f32)) {
        let (uw, uh) = self.ui_dims();
        ((vec2(uw - 100.0, uh - 190.0), 66.0), (vec2(uw - 240.0, uh - 120.0), 54.0))
    }

    fn touch_joy_region(&self) -> Rect {
        let (uw, uh) = self.ui_dims();
        Rect::new(0.0, 70.0, uw * 0.42, uh - 70.0)
    }

    fn touch_joy_rest(&self) -> Vec2 {
        let (_, uh) = self.ui_dims();
        vec2(140.0, uh - 150.0)
    }

    /// Le os dedos na tela e devolve os mesmos `Controls` que o teclado/mouse devolveriam.
    fn read_touch(&mut self, dt: f32) -> Controls {
        let s = ui_scale();
        let ts = touches();
        let mut c = Controls::none();
        let menu_ui = self.paused();
        let container = matches!(self.ui, UiMode::Inventory | UiMode::Workbench);
        let in_game = self.ui == UiMode::None;

        // procura um dedo pelo id: (posicao em pixels, terminou?)
        let find = |id: u64| -> Option<(Vec2, bool)> {
            ts.iter().find(|t| t.id == id).map(|t| {
                (
                    t.position,
                    matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled),
                )
            })
        };

        let ((atk_c, atk_r), (use_c, use_r)) = self.touch_action_circles();
        let buttons = if menu_ui { Vec::new() } else { self.touch_buttons() };
        let joy_region = self.touch_joy_region();
        let hb = self.hotbar_origin();

        // ---- dedos que acabaram de encostar: decidem o que controlam
        for t in ts.iter() {
            if !matches!(t.phase, TouchPhase::Started) {
                continue;
            }
            let p = t.position / s;

            if menu_ui {
                if self.touch.ptr_id.is_none() {
                    self.touch.ptr_id = Some(t.id);
                    self.touch.ptr_t = 0.0;
                    self.touch.ptr_long = false;
                    c.attack_pressed = true; // menus respondem ao encostar
                    c.pointer = t.position;
                    self.touch.last_ptr = t.position;
                }
                continue;
            }

            if let Some((_, b)) = buttons.iter().find(|(r, _)| r.contains(p)) {
                match *b {
                    TouchBtn::Menu => c.cancel = true,
                    TouchBtn::Inv => c.toggle_inventory = true,
                    TouchBtn::Hurt => c.test_hurt = true,
                    TouchBtn::Debug => c.toggle_debug = true,
                    TouchBtn::Gallery => c.toggle_gallery = true,
                    TouchBtn::ZoomIn => c.zoom_delta = 1,
                    TouchBtn::ZoomOut => c.zoom_delta = -1,
                    TouchBtn::Drop => c.drop_item = true,
                    TouchBtn::Floor => self.touch.floor_latched = !self.touch.floor_latched,
                    TouchBtn::Help => c.toggle_help = true,
                }
                continue;
            }

            if container {
                if self.touch.ptr_id.is_none() {
                    self.touch.ptr_id = Some(t.id);
                    self.touch.ptr_t = 0.0;
                    self.touch.ptr_long = false;
                    self.touch.last_ptr = t.position;
                }
                continue;
            }

            // ---- em jogo
            let mut hit_slot: Option<usize> = None;
            for i in 0..9usize {
                let r = Rect::new(hb.x + i as f32 * (UI_SLOT + UI_GAP), hb.y, UI_SLOT, UI_SLOT);
                if r.contains(p) {
                    hit_slot = Some(i);
                }
            }
            if let Some(i) = hit_slot {
                c.slot_select = Some(i);
                continue;
            }
            if (p - atk_c).length() <= atk_r + 10.0 {
                self.touch.atk_id = Some(t.id);
                c.attack_pressed = true;
                continue;
            }
            if (p - use_c).length() <= use_r + 10.0 {
                self.touch.use_id = Some(t.id);
                c.use_pressed = true;
                continue;
            }
            if joy_region.contains(p) && self.touch.joy_id.is_none() {
                self.touch.joy_id = Some(t.id);
                self.touch.joy_origin = p;
                self.touch.joy_pos = p;
                continue;
            }
            if self.touch.aim_id.is_none() {
                self.touch.aim_id = Some(t.id);
            }
        }

        // ---- joystick: 8 direcoes
        if let Some(id) = self.touch.joy_id {
            match find(id) {
                Some((pos, ended)) if !ended => self.touch.joy_pos = pos / s,
                _ => self.touch.joy_id = None,
            }
        }
        if self.touch.joy_id.is_some() {
            let v = self.touch.joy_pos - self.touch.joy_origin;
            if v.length() > 16.0 {
                let a = (v.y.atan2(v.x) / (PI / 4.0)).round() * (PI / 4.0);
                c.move_axis = vec2(a.cos(), a.sin());
            }
        }

        // ---- BATER (segurar). Arrastar o dedo no botao mira.
        if let Some(id) = self.touch.atk_id {
            match find(id) {
                Some((pos, ended)) if !ended => {
                    c.attack_down = true;
                    let d = pos / s - atk_c;
                    if d.length() > 18.0 {
                        self.touch.aim = d.normalize();
                    }
                }
                _ => self.touch.atk_id = None,
            }
        }

        // ---- USAR / COLOCAR
        if let Some(id) = self.touch.use_id {
            match find(id) {
                Some((pos, ended)) if !ended => {
                    c.use_down = true;
                    let d = pos / s - use_c;
                    if d.length() > 18.0 {
                        self.touch.aim = d.normalize();
                    }
                }
                _ => self.touch.use_id = None,
            }
        }

        // ---- dedo no mundo: mira naquela direcao (como o mouse)
        if let Some(id) = self.touch.aim_id {
            match find(id) {
                Some((pos, ended)) if !ended => {
                    let torso = self.to_screen(self.player.pos + vec2(0.0, -0.45));
                    let d = pos - torso;
                    if d.length() > 8.0 {
                        self.touch.aim = d.normalize();
                    }
                }
                _ => self.touch.aim_id = None,
            }
        }

        // ---- dedo "mouse" dos menus e do inventario
        c.pointer = self.touch.last_ptr;
        if let Some(id) = self.touch.ptr_id {
            match find(id) {
                Some((pos, ended)) => {
                    c.pointer = pos;
                    self.touch.last_ptr = pos;
                    if container {
                        // toque curto = clique esquerdo, segurar = clique direito
                        self.touch.ptr_t += dt;
                        if !self.touch.ptr_long && self.touch.ptr_t > 0.45 {
                            c.use_pressed = true;
                            self.touch.ptr_long = true;
                        }
                        if ended {
                            if !self.touch.ptr_long {
                                c.attack_pressed = true;
                            }
                            self.touch.ptr_id = None;
                        }
                    } else {
                        c.attack_down = !ended;
                        if ended {
                            self.touch.ptr_id = None;
                        }
                    }
                }
                None => self.touch.ptr_id = None,
            }
        }

        c.aim_override = Some(self.touch.aim);
        c.place_floor = self.touch.floor_latched;
        c
    }

    /// Desenha o joystick, os botoes de BATER/USAR e os botoes do topo.
    fn draw_touch_hud(&self) {
        let in_game = self.ui == UiMode::None;
        let container = matches!(self.ui, UiMode::Inventory | UiMode::Workbench);
        if !(in_game || container) {
            return;
        }

        // botoes do topo (todas as teclas do PC)
        for (r, b) in self.touch_buttons() {
            let active = match b {
                TouchBtn::Floor => self.touch.floor_latched,
                TouchBtn::Debug => self.show_debug,
                TouchBtn::Help => self.show_controls,
                _ => false,
            };
            let bg = if active {
                Color::new(0.28, 0.5, 0.34, 0.9)
            } else {
                Color::new(0.1, 0.1, 0.14, 0.72)
            };
            draw_rectangle(r.x, r.y, r.w, r.h, bg);
            draw_rectangle_lines(r.x, r.y, r.w, r.h, 2.0, Color::new(0.7, 0.7, 0.8, 0.8));
            // no celular de verdade: rotulos curtos; no modo celular do PC: o icone e a TECLA
            let lab = if self.touch_ui { touch_label(b) } else { key_label(b) };
            text_centered(lab, r.x + r.w / 2.0, r.y + r.h / 2.0 + 7.0, 22.0, WHITE);
        }
        if !in_game {
            return;
        }

        // joystick (flutuante: nasce onde voce encosta)
        let (base, knob) = if self.touch.joy_id.is_some() {
            let o = self.touch.joy_origin;
            let v = self.touch.joy_pos - o;
            let k = if v.length() > 60.0 { o + v.normalize() * 60.0 } else { self.touch.joy_pos };
            (o, k)
        } else {
            let r = self.touch_joy_rest();
            (r, r)
        };
        draw_circle(base.x, base.y, 70.0, Color::new(1.0, 1.0, 1.0, 0.10));
        draw_circle_lines(base.x, base.y, 70.0, 3.0, Color::new(1.0, 1.0, 1.0, 0.35));
        draw_circle(knob.x, knob.y, 30.0, Color::new(1.0, 1.0, 1.0, 0.35));

        // BATER e USAR
        let ((atk_c, atk_r), (use_c, use_r)) = self.touch_action_circles();
        let atk_on = self.touch.atk_id.is_some();
        let use_on = self.touch.use_id.is_some();
        draw_circle(
            atk_c.x,
            atk_c.y,
            atk_r,
            if atk_on { Color::new(0.85, 0.3, 0.3, 0.6) } else { Color::new(0.85, 0.3, 0.3, 0.32) },
        );
        draw_circle_lines(atk_c.x, atk_c.y, atk_r, 3.0, Color::new(1.0, 1.0, 1.0, 0.6));
        text_centered("BATER", atk_c.x, atk_c.y + 8.0, 26.0, WHITE);
        if !self.touch_ui {
            text_centered("[Mouse Esq]", atk_c.x, atk_c.y + 30.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.8));
        }
        draw_circle(
            use_c.x,
            use_c.y,
            use_r,
            if use_on { Color::new(0.3, 0.6, 0.9, 0.6) } else { Color::new(0.3, 0.6, 0.9, 0.32) },
        );
        draw_circle_lines(use_c.x, use_c.y, use_r, 3.0, Color::new(1.0, 1.0, 1.0, 0.6));
        text_centered("USAR", use_c.x, use_c.y + 8.0, 24.0, WHITE);
        if !self.touch_ui {
            text_centered("[Mouse Dir]", use_c.x, use_c.y + 28.0, 15.0, Color::new(1.0, 1.0, 1.0, 0.8));
        }
    }
}

// =====================================================================
// SECAO 14 - MODO CELULAR NO PC (TESTE) E LISTA DE CONTROLES (tecla H)
//
//  Ligue em: Esc -> Configuracoes -> "Modo celular (teste)": Sim/Nao
//  (padrao: Nao). Pensado para rodar o jogo num emulador de Windows no
//  celular (Winlator / GameHub), onde o toque na tela vira clique de mouse.
//
//  * Os botoes aparecem na tela e o icone de cada um e a TECLA do PC.
//  * O MOUSE (ou o toque emulado) faz o papel do dedo: clique nos botoes,
//    na hotbar e nos slots do inventario. O teclado continua funcionando.
//  * MIRA AUTOMATICA: o bloco alvo fica sempre na direcao em que o
//    personagem anda. Clicar (e segurar) no mundo vira a mira para aquele lado.
//  * Inventario/bancada: clique = botao esquerdo, segurar = botao direito.
//  * Tecla H (ou botao ?) mostra/esconde a lista de controles.
// =====================================================================

/// Rotulo dos botoes quando o icone deve ser a TECLA do PC.
fn key_label(b: TouchBtn) -> &'static str {
    match b {
        TouchBtn::Floor => "Shift",
        TouchBtn::Drop => "Q",
        TouchBtn::ZoomOut => "-",
        TouchBtn::ZoomIn => "+",
        TouchBtn::Gallery => "F1",
        TouchBtn::Debug => "F3",
        TouchBtn::Hurt => "F4",
        TouchBtn::Inv => "E",
        TouchBtn::Menu => "Esc",
        TouchBtn::Help => "H",
    }
}

impl Game {
    /// Controles do MODO CELULAR no PC: o mouse faz o papel do dedo e o teclado
    /// continua valendo. Devolve a mesma struct `Controls` de sempre.
    fn read_mobile_pc(&mut self, dt: f32) -> Controls {
        let mut c = read_controls();
        let s = ui_scale();
        let (mx, my) = mouse_position();
        let mpos = vec2(mx, my);
        let p = mpos / s;
        let pressed = is_mouse_button_pressed(MouseButton::Left);
        let down = is_mouse_button_down(MouseButton::Left);
        let right_click = is_mouse_button_pressed(MouseButton::Right);
        let menu_ui = self.paused();
        let container = matches!(self.ui, UiMode::Inventory | UiMode::Workbench);
        let in_game = self.ui == UiMode::None;

        // soltou o botao do mouse: tudo que estava "segurado" termina
        if !down {
            self.touch.joy_id = None;
            self.touch.atk_id = None;
            self.touch.use_id = None;
            self.touch.aim_id = None;
        }
        if !container {
            self.touch.ptr_id = None;
        }

        // ---- botoes do topo (as teclas): valem no jogo e no inventario
        let mut on_button = false;
        if pressed && !menu_ui {
            let buttons = self.touch_buttons();
            if let Some((_, b)) = buttons.iter().find(|(r, _)| r.contains(p)) {
                match *b {
                    TouchBtn::Menu => c.cancel = true,
                    TouchBtn::Inv => c.toggle_inventory = true,
                    TouchBtn::Hurt => c.test_hurt = true,
                    TouchBtn::Debug => c.toggle_debug = true,
                    TouchBtn::Gallery => c.toggle_gallery = true,
                    TouchBtn::ZoomIn => c.zoom_delta = 1,
                    TouchBtn::ZoomOut => c.zoom_delta = -1,
                    TouchBtn::Drop => c.drop_item = true,
                    TouchBtn::Floor => self.touch.floor_latched = !self.touch.floor_latched,
                    TouchBtn::Help => c.toggle_help = true,
                }
                on_button = true;
            }
        }

        if in_game {
            // o botao esquerdo do mouse so age pelos botoes na tela
            c.attack_down = false;
            c.attack_pressed = false;

            if pressed && !on_button {
                let hb = self.hotbar_origin();
                let ((atk_c, atk_r), (use_c, use_r)) = self.touch_action_circles();
                let joy_region = self.touch_joy_region();
                let mut hit_slot: Option<usize> = None;
                for i in 0..9usize {
                    let r = Rect::new(hb.x + i as f32 * (UI_SLOT + UI_GAP), hb.y, UI_SLOT, UI_SLOT);
                    if r.contains(p) {
                        hit_slot = Some(i);
                    }
                }
                if let Some(i) = hit_slot {
                    c.slot_select = Some(i);
                } else if (p - atk_c).length() <= atk_r + 10.0 {
                    self.touch.atk_id = Some(0);
                    c.attack_pressed = true;
                } else if (p - use_c).length() <= use_r + 10.0 {
                    self.touch.use_id = Some(0);
                    c.use_pressed = true;
                } else if joy_region.contains(p) {
                    self.touch.joy_id = Some(0);
                    self.touch.joy_origin = p;
                    self.touch.joy_pos = p;
                } else {
                    self.touch.aim_id = Some(0);
                }
            }

            // joystick virtual (arraste com o mouse): 8 direcoes
            if self.touch.joy_id.is_some() && down {
                self.touch.joy_pos = p;
                let v = self.touch.joy_pos - self.touch.joy_origin;
                if v.length() > 16.0 {
                    let a = (v.y.atan2(v.x) / (PI / 4.0)).round() * (PI / 4.0);
                    c.move_axis = vec2(a.cos(), a.sin());
                }
            }
            if self.touch.atk_id.is_some() && down {
                c.attack_down = true;
            }
            if self.touch.use_id.is_some() && down {
                c.use_down = true;
            }

            // clicar e segurar no mundo: vira a mira para aquele lado
            if self.touch.aim_id.is_some() && down {
                let torso = self.to_screen(self.player.pos + vec2(0.0, -0.45));
                let d = mpos - torso;
                if d.length() > 8.0 {
                    self.touch.aim = d.normalize();
                }
            }

            // MIRA AUTOMATICA: o alvo acompanha a direcao em que voce anda
            if c.move_axis.length() > 0.1 && self.touch.aim_id.is_none() {
                self.touch.aim = c.move_axis.normalize();
            }
            c.aim_override = Some(self.touch.aim);
            c.place_floor = c.place_floor || self.touch.floor_latched;
        } else if container {
            // clique curto = botao esquerdo, segurar = botao direito (como no toque)
            c.attack_pressed = false;
            c.use_pressed = right_click;
            if pressed && !on_button {
                self.touch.ptr_id = Some(0);
                self.touch.ptr_t = 0.0;
                self.touch.ptr_long = false;
            }
            if self.touch.ptr_id.is_some() {
                if down {
                    self.touch.ptr_t += dt;
                    if !self.touch.ptr_long && self.touch.ptr_t > 0.45 {
                        c.use_pressed = true;
                        self.touch.ptr_long = true;
                    }
                } else {
                    if !self.touch.ptr_long {
                        c.attack_pressed = true;
                    }
                    self.touch.ptr_id = None;
                }
            }
        }
        c
    }

    /// Lista de controles (tecla H / botao ?). Desenhada em coordenadas virtuais da interface.
    fn draw_controls_help(&self) {
        let (uw, uh) = self.ui_dims();
        let pw = 780.0f32.min(uw - 16.0);
        let ph = 490.0f32.min(uh - 16.0);
        let x = (uw - pw) / 2.0;
        let y = (uh - ph) / 2.0;
        draw_rectangle(x, y, pw, ph, Color::new(0.05, 0.05, 0.08, 0.95));
        draw_rectangle_lines(x, y, pw, ph, 3.0, Color::new(0.7, 0.7, 0.85, 1.0));
        text_centered("CONTROLES  (H ou Esc fecha)", x + pw / 2.0, y + 40.0, 30.0, WHITE);

        let mobile = self.touch_ui || self.settings.mobile_mode;
        let rows: Vec<(&str, &str)> = if mobile {
            vec![
                ("Joystick / WASD", "andar (8 direcoes)"),
                ("Mira", "automatica: bloco na direcao em que voce anda"),
                ("Clicar no mundo", "vira a mira para aquele lado"),
                ("BATER (segure)", "quebrar o bloco mirado"),
                ("USAR", "colocar / usar (bancada, porta)"),
                ("Shift (botao)", "colocar como piso (liga/desliga)"),
                ("Hotbar / 1-9", "escolher o item"),
                ("E", "inventario e crafting"),
                ("Q", "soltar 1 item"),
                ("+ / -", "zoom"),
                ("Esc", "menu (personagem, configuracoes)"),
                ("F1 / F3 / F4", "galeria / debug / teste de dano"),
                ("H ou ?", "mostrar / esconder esta lista"),
            ]
        } else {
            vec![
                ("WASD / Setas", "andar (8 direcoes)"),
                ("Mouse", "mirar"),
                ("Botao esquerdo (segure)", "quebrar"),
                ("Botao direito", "colocar / usar (bancada, porta)"),
                ("Shift + direito", "colocar como piso"),
                ("1-9 / roda do mouse", "escolher o item"),
                ("E", "inventario e crafting"),
                ("Q", "soltar 1 item"),
                ("+ / -", "zoom"),
                ("Esc", "menu (personagem, configuracoes)"),
                ("F1", "galeria de sprites"),
                ("F3", "debug"),
                ("F4", "teste de dano"),
                ("H", "mostrar / esconder esta lista"),
            ]
        };
        let key_col = Color::new(1.0, 0.9, 0.5, 1.0);
        for (i, (k, d)) in rows.iter().enumerate() {
            let ry = y + 78.0 + i as f32 * 28.0;
            text_shadow(k, x + 28.0, ry, 22.0, key_col);
            text_shadow(d, x + 290.0, ry, 22.0, WHITE);
        }
    }
}
