//! This crate provides a type `Ordinal<T>`, that formats an integer as an ordinal number.
//!
//! ## Install
//!
//! Command:
//!
//! ```sh
//! cargo add ordinal-type
//! ```
//!
//! Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! ordinal-type = "*"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use ordinal_type::Ordinal;
//!
//! let ordinal = Ordinal(1);
//! assert_eq!(ordinal.to_string(), "1st");
//! ```
//!

use num_integer::Integer;
use num_traits::ToPrimitive;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ordinal<T>(pub T);

impl<T> Display for Ordinal<T>
where
    T: Integer + Display + ToPrimitive + Clone,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.suffix())
    }
}

impl<T> Ordinal<T>
where
    T: Integer + Display + ToPrimitive + Clone,
{
    /// Returns the suffix of the ordinal number.
    /// For example, `1` returns `"st"`, `2` returns `"nd"`, `3` returns `"rd"`, and `4` returns `"th"`.
    /// This method is useful when you want to format the ordinal number yourself.
    /// ```rust
    /// use ordinal_type::Ordinal;
    ///
    /// let ordinal = Ordinal(1);
    /// assert_eq!(ordinal.suffix(), "st");
    /// ```
    pub fn suffix(&self) -> &'static str {
        let last_digits: String = self.0.to_string();
        if last_digits.ends_with('1') && !last_digits.ends_with("11") {
            "st"
        } else if last_digits.ends_with('2') && !last_digits.ends_with("12") {
            "nd"
        } else if last_digits.ends_with('3') && !last_digits.ends_with("13") {
            "rd"
        } else {
            "th"
        }
    }

    /// Returns the primitive value of the ordinal number.
    pub fn to_primitive(&self) -> T {
        self.0.clone()
    }

    /// Returns the primitive value of the ordinal number as a `u8`.
    pub fn to_u8(&self) -> u8 {
        self.0.to_u8().expect("Failed to convert to u8")
    }

    /// Returns the primitive value of the ordinal number as a `u16`.
    pub fn to_u16(&self) -> u16 {
        self.0.to_u16().expect("Failed to convert to u16")
    }

    /// Returns the primitive value of the ordinal number as a `u32`.
    pub fn to_u32(&self) -> u32 {
        self.0.to_u32().expect("Failed to convert to u32")
    }

    /// Returns the primitive value of the ordinal number as a `u64`.
    pub fn to_u64(&self) -> u64 {
        self.0.to_u64().expect("Failed to convert to u64")
    }

    /// Returns the primitive value of the ordinal number as a `u128`.
    pub fn to_u128(&self) -> u128 {
        self.0.to_u128().expect("Failed to convert to u128")
    }

    /// Returns the primitive value of the ordinal number as a `usize`.
    pub fn to_usize(&self) -> usize {
        self.0.to_usize().expect("Failed to convert to usize")
    }

    /// Returns the primitive value of the ordinal number as a `i8`.
    pub fn to_i8(&self) -> i8 {
        self.0.to_i8().expect("Failed to convert to i8")
    }

    /// Returns the primitive value of the ordinal number as a `i16`.
    pub fn to_i16(&self) -> i16 {
        self.0.to_i16().expect("Failed to convert to i16")
    }

    /// Returns the primitive value of the ordinal number as a `i32`.
    pub fn to_i32(&self) -> i32 {
        self.0.to_i32().expect("Failed to convert to i32")
    }

    /// Returns the primitive value of the ordinal number as a `i64`.
    pub fn to_i64(&self) -> i64 {
        self.0.to_i64().expect("Failed to convert to i64")
    }

    /// Returns the primitive value of the ordinal number as a `i128`.
    pub fn to_i128(&self) -> i128 {
        self.0.to_i128().expect("Failed to convert to i128")
    }

    /// Returns the primitive value of the ordinal number as a `isize`.
    pub fn to_isize(&self) -> isize {
        self.0.to_isize().expect("Failed to convert to isize")
    }
}

trait ToOrdinal {
    fn to_ordinal(&self) -> Ordinal<Self>
    where
        Self: Sized;
}

macro_rules! impl_to_ordinal_for_integers {
    ($($t:ty),*) => {
        $(
            impl ToOrdinal for $t {
                fn to_ordinal(&self) -> Ordinal<$t> {
                    Ordinal(*self)
                }
            }
        )*
    };
}

impl_to_ordinal_for_integers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use crate::{Ordinal, ToOrdinal};
    use num_bigint::{BigInt, BigUint};

    #[test]
    fn test_positive_ordinals() {
        let positive_numbers: [u16; 1000] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
            110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
            127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
            144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
            161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177,
            178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194,
            195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
            212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228,
            229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245,
            246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262,
            263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279,
            280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296,
            297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313,
            314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330,
            331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346, 347,
            348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364,
            365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381,
            382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398,
            399, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415,
            416, 417, 418, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432,
            433, 434, 435, 436, 437, 438, 439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449,
            450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466,
            467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483,
            484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500,
            501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517,
            518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
            535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551,
            552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568,
            569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585,
            586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602,
            603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614, 615, 616, 617, 618, 619,
            620, 621, 622, 623, 624, 625, 626, 627, 628, 629, 630, 631, 632, 633, 634, 635, 636,
            637, 638, 639, 640, 641, 642, 643, 644, 645, 646, 647, 648, 649, 650, 651, 652, 653,
            654, 655, 656, 657, 658, 659, 660, 661, 662, 663, 664, 665, 666, 667, 668, 669, 670,
            671, 672, 673, 674, 675, 676, 677, 678, 679, 680, 681, 682, 683, 684, 685, 686, 687,
            688, 689, 690, 691, 692, 693, 694, 695, 696, 697, 698, 699, 700, 701, 702, 703, 704,
            705, 706, 707, 708, 709, 710, 711, 712, 713, 714, 715, 716, 717, 718, 719, 720, 721,
            722, 723, 724, 725, 726, 727, 728, 729, 730, 731, 732, 733, 734, 735, 736, 737, 738,
            739, 740, 741, 742, 743, 744, 745, 746, 747, 748, 749, 750, 751, 752, 753, 754, 755,
            756, 757, 758, 759, 760, 761, 762, 763, 764, 765, 766, 767, 768, 769, 770, 771, 772,
            773, 774, 775, 776, 777, 778, 779, 780, 781, 782, 783, 784, 785, 786, 787, 788, 789,
            790, 791, 792, 793, 794, 795, 796, 797, 798, 799, 800, 801, 802, 803, 804, 805, 806,
            807, 808, 809, 810, 811, 812, 813, 814, 815, 816, 817, 818, 819, 820, 821, 822, 823,
            824, 825, 826, 827, 828, 829, 830, 831, 832, 833, 834, 835, 836, 837, 838, 839, 840,
            841, 842, 843, 844, 845, 846, 847, 848, 849, 850, 851, 852, 853, 854, 855, 856, 857,
            858, 859, 860, 861, 862, 863, 864, 865, 866, 867, 868, 869, 870, 871, 872, 873, 874,
            875, 876, 877, 878, 879, 880, 881, 882, 883, 884, 885, 886, 887, 888, 889, 890, 891,
            892, 893, 894, 895, 896, 897, 898, 899, 900, 901, 902, 903, 904, 905, 906, 907, 908,
            909, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919, 920, 921, 922, 923, 924, 925,
            926, 927, 928, 929, 930, 931, 932, 933, 934, 935, 936, 937, 938, 939, 940, 941, 942,
            943, 944, 945, 946, 947, 948, 949, 950, 951, 952, 953, 954, 955, 956, 957, 958, 959,
            960, 961, 962, 963, 964, 965, 966, 967, 968, 969, 970, 971, 972, 973, 974, 975, 976,
            977, 978, 979, 980, 981, 982, 983, 984, 985, 986, 987, 988, 989, 990, 991, 992, 993,
            994, 995, 996, 997, 998, 999, 1000,
        ];
        let positive_ordinals: [&str; 1000] = [
            "1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th",
            "13th", "14th", "15th", "16th", "17th", "18th", "19th", "20th", "21st", "22nd", "23rd",
            "24th", "25th", "26th", "27th", "28th", "29th", "30th", "31st", "32nd", "33rd", "34th",
            "35th", "36th", "37th", "38th", "39th", "40th", "41st", "42nd", "43rd", "44th", "45th",
            "46th", "47th", "48th", "49th", "50th", "51st", "52nd", "53rd", "54th", "55th", "56th",
            "57th", "58th", "59th", "60th", "61st", "62nd", "63rd", "64th", "65th", "66th", "67th",
            "68th", "69th", "70th", "71st", "72nd", "73rd", "74th", "75th", "76th", "77th", "78th",
            "79th", "80th", "81st", "82nd", "83rd", "84th", "85th", "86th", "87th", "88th", "89th",
            "90th", "91st", "92nd", "93rd", "94th", "95th", "96th", "97th", "98th", "99th",
            "100th", "101st", "102nd", "103rd", "104th", "105th", "106th", "107th", "108th",
            "109th", "110th", "111th", "112th", "113th", "114th", "115th", "116th", "117th",
            "118th", "119th", "120th", "121st", "122nd", "123rd", "124th", "125th", "126th",
            "127th", "128th", "129th", "130th", "131st", "132nd", "133rd", "134th", "135th",
            "136th", "137th", "138th", "139th", "140th", "141st", "142nd", "143rd", "144th",
            "145th", "146th", "147th", "148th", "149th", "150th", "151st", "152nd", "153rd",
            "154th", "155th", "156th", "157th", "158th", "159th", "160th", "161st", "162nd",
            "163rd", "164th", "165th", "166th", "167th", "168th", "169th", "170th", "171st",
            "172nd", "173rd", "174th", "175th", "176th", "177th", "178th", "179th", "180th",
            "181st", "182nd", "183rd", "184th", "185th", "186th", "187th", "188th", "189th",
            "190th", "191st", "192nd", "193rd", "194th", "195th", "196th", "197th", "198th",
            "199th", "200th", "201st", "202nd", "203rd", "204th", "205th", "206th", "207th",
            "208th", "209th", "210th", "211th", "212th", "213th", "214th", "215th", "216th",
            "217th", "218th", "219th", "220th", "221st", "222nd", "223rd", "224th", "225th",
            "226th", "227th", "228th", "229th", "230th", "231st", "232nd", "233rd", "234th",
            "235th", "236th", "237th", "238th", "239th", "240th", "241st", "242nd", "243rd",
            "244th", "245th", "246th", "247th", "248th", "249th", "250th", "251st", "252nd",
            "253rd", "254th", "255th", "256th", "257th", "258th", "259th", "260th", "261st",
            "262nd", "263rd", "264th", "265th", "266th", "267th", "268th", "269th", "270th",
            "271st", "272nd", "273rd", "274th", "275th", "276th", "277th", "278th", "279th",
            "280th", "281st", "282nd", "283rd", "284th", "285th", "286th", "287th", "288th",
            "289th", "290th", "291st", "292nd", "293rd", "294th", "295th", "296th", "297th",
            "298th", "299th", "300th", "301st", "302nd", "303rd", "304th", "305th", "306th",
            "307th", "308th", "309th", "310th", "311th", "312th", "313th", "314th", "315th",
            "316th", "317th", "318th", "319th", "320th", "321st", "322nd", "323rd", "324th",
            "325th", "326th", "327th", "328th", "329th", "330th", "331st", "332nd", "333rd",
            "334th", "335th", "336th", "337th", "338th", "339th", "340th", "341st", "342nd",
            "343rd", "344th", "345th", "346th", "347th", "348th", "349th", "350th", "351st",
            "352nd", "353rd", "354th", "355th", "356th", "357th", "358th", "359th", "360th",
            "361st", "362nd", "363rd", "364th", "365th", "366th", "367th", "368th", "369th",
            "370th", "371st", "372nd", "373rd", "374th", "375th", "376th", "377th", "378th",
            "379th", "380th", "381st", "382nd", "383rd", "384th", "385th", "386th", "387th",
            "388th", "389th", "390th", "391st", "392nd", "393rd", "394th", "395th", "396th",
            "397th", "398th", "399th", "400th", "401st", "402nd", "403rd", "404th", "405th",
            "406th", "407th", "408th", "409th", "410th", "411th", "412th", "413th", "414th",
            "415th", "416th", "417th", "418th", "419th", "420th", "421st", "422nd", "423rd",
            "424th", "425th", "426th", "427th", "428th", "429th", "430th", "431st", "432nd",
            "433rd", "434th", "435th", "436th", "437th", "438th", "439th", "440th", "441st",
            "442nd", "443rd", "444th", "445th", "446th", "447th", "448th", "449th", "450th",
            "451st", "452nd", "453rd", "454th", "455th", "456th", "457th", "458th", "459th",
            "460th", "461st", "462nd", "463rd", "464th", "465th", "466th", "467th", "468th",
            "469th", "470th", "471st", "472nd", "473rd", "474th", "475th", "476th", "477th",
            "478th", "479th", "480th", "481st", "482nd", "483rd", "484th", "485th", "486th",
            "487th", "488th", "489th", "490th", "491st", "492nd", "493rd", "494th", "495th",
            "496th", "497th", "498th", "499th", "500th", "501st", "502nd", "503rd", "504th",
            "505th", "506th", "507th", "508th", "509th", "510th", "511th", "512th", "513th",
            "514th", "515th", "516th", "517th", "518th", "519th", "520th", "521st", "522nd",
            "523rd", "524th", "525th", "526th", "527th", "528th", "529th", "530th", "531st",
            "532nd", "533rd", "534th", "535th", "536th", "537th", "538th", "539th", "540th",
            "541st", "542nd", "543rd", "544th", "545th", "546th", "547th", "548th", "549th",
            "550th", "551st", "552nd", "553rd", "554th", "555th", "556th", "557th", "558th",
            "559th", "560th", "561st", "562nd", "563rd", "564th", "565th", "566th", "567th",
            "568th", "569th", "570th", "571st", "572nd", "573rd", "574th", "575th", "576th",
            "577th", "578th", "579th", "580th", "581st", "582nd", "583rd", "584th", "585th",
            "586th", "587th", "588th", "589th", "590th", "591st", "592nd", "593rd", "594th",
            "595th", "596th", "597th", "598th", "599th", "600th", "601st", "602nd", "603rd",
            "604th", "605th", "606th", "607th", "608th", "609th", "610th", "611th", "612th",
            "613th", "614th", "615th", "616th", "617th", "618th", "619th", "620th", "621st",
            "622nd", "623rd", "624th", "625th", "626th", "627th", "628th", "629th", "630th",
            "631st", "632nd", "633rd", "634th", "635th", "636th", "637th", "638th", "639th",
            "640th", "641st", "642nd", "643rd", "644th", "645th", "646th", "647th", "648th",
            "649th", "650th", "651st", "652nd", "653rd", "654th", "655th", "656th", "657th",
            "658th", "659th", "660th", "661st", "662nd", "663rd", "664th", "665th", "666th",
            "667th", "668th", "669th", "670th", "671st", "672nd", "673rd", "674th", "675th",
            "676th", "677th", "678th", "679th", "680th", "681st", "682nd", "683rd", "684th",
            "685th", "686th", "687th", "688th", "689th", "690th", "691st", "692nd", "693rd",
            "694th", "695th", "696th", "697th", "698th", "699th", "700th", "701st", "702nd",
            "703rd", "704th", "705th", "706th", "707th", "708th", "709th", "710th", "711th",
            "712th", "713th", "714th", "715th", "716th", "717th", "718th", "719th", "720th",
            "721st", "722nd", "723rd", "724th", "725th", "726th", "727th", "728th", "729th",
            "730th", "731st", "732nd", "733rd", "734th", "735th", "736th", "737th", "738th",
            "739th", "740th", "741st", "742nd", "743rd", "744th", "745th", "746th", "747th",
            "748th", "749th", "750th", "751st", "752nd", "753rd", "754th", "755th", "756th",
            "757th", "758th", "759th", "760th", "761st", "762nd", "763rd", "764th", "765th",
            "766th", "767th", "768th", "769th", "770th", "771st", "772nd", "773rd", "774th",
            "775th", "776th", "777th", "778th", "779th", "780th", "781st", "782nd", "783rd",
            "784th", "785th", "786th", "787th", "788th", "789th", "790th", "791st", "792nd",
            "793rd", "794th", "795th", "796th", "797th", "798th", "799th", "800th", "801st",
            "802nd", "803rd", "804th", "805th", "806th", "807th", "808th", "809th", "810th",
            "811th", "812th", "813th", "814th", "815th", "816th", "817th", "818th", "819th",
            "820th", "821st", "822nd", "823rd", "824th", "825th", "826th", "827th", "828th",
            "829th", "830th", "831st", "832nd", "833rd", "834th", "835th", "836th", "837th",
            "838th", "839th", "840th", "841st", "842nd", "843rd", "844th", "845th", "846th",
            "847th", "848th", "849th", "850th", "851st", "852nd", "853rd", "854th", "855th",
            "856th", "857th", "858th", "859th", "860th", "861st", "862nd", "863rd", "864th",
            "865th", "866th", "867th", "868th", "869th", "870th", "871st", "872nd", "873rd",
            "874th", "875th", "876th", "877th", "878th", "879th", "880th", "881st", "882nd",
            "883rd", "884th", "885th", "886th", "887th", "888th", "889th", "890th", "891st",
            "892nd", "893rd", "894th", "895th", "896th", "897th", "898th", "899th", "900th",
            "901st", "902nd", "903rd", "904th", "905th", "906th", "907th", "908th", "909th",
            "910th", "911th", "912th", "913th", "914th", "915th", "916th", "917th", "918th",
            "919th", "920th", "921st", "922nd", "923rd", "924th", "925th", "926th", "927th",
            "928th", "929th", "930th", "931st", "932nd", "933rd", "934th", "935th", "936th",
            "937th", "938th", "939th", "940th", "941st", "942nd", "943rd", "944th", "945th",
            "946th", "947th", "948th", "949th", "950th", "951st", "952nd", "953rd", "954th",
            "955th", "956th", "957th", "958th", "959th", "960th", "961st", "962nd", "963rd",
            "964th", "965th", "966th", "967th", "968th", "969th", "970th", "971st", "972nd",
            "973rd", "974th", "975th", "976th", "977th", "978th", "979th", "980th", "981st",
            "982nd", "983rd", "984th", "985th", "986th", "987th", "988th", "989th", "990th",
            "991st", "992nd", "993rd", "994th", "995th", "996th", "997th", "998th", "999th",
            "1000th",
        ];

        for i in 0..1000 {
            assert_eq!(
                Ordinal(positive_numbers[i]).to_string(),
                positive_ordinals[i]
            );
        }
    }

    #[test]
    fn test_negative_ordinals() {
        let negative_numbers: [i8; 10] = [-1, -2, -3, -4, -5, -6, -7, -8, -9, -10];
        let negative_ordinals: [&str; 10] = [
            "-1st", "-2nd", "-3rd", "-4th", "-5th", "-6th", "-7th", "-8th", "-9th", "-10th",
        ];

        for i in 0..10 {
            assert_eq!(
                Ordinal(negative_numbers[i]).to_string(),
                negative_ordinals[i]
            );
        }
    }

    #[test]
    fn test_types() {
        let types: (
            i8,
            i16,
            i32,
            i64,
            i128,
            isize,
            u8,
            u16,
            u32,
            u64,
            u128,
            usize,
        ) = (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1);

        let big_types: (BigInt, BigUint) = ((1 as i8).into(), (1 as u8).into());

        assert_eq!("1st", Ordinal(types.0).to_string());
        assert_eq!("1st", Ordinal(types.1).to_string());
        assert_eq!("1st", Ordinal(types.2).to_string());
        assert_eq!("1st", Ordinal(types.3).to_string());
        assert_eq!("1st", Ordinal(types.4).to_string());
        assert_eq!("1st", Ordinal(types.5).to_string());
        assert_eq!("1st", Ordinal(types.6).to_string());
        assert_eq!("1st", Ordinal(types.7).to_string());
        assert_eq!("1st", Ordinal(types.8).to_string());
        assert_eq!("1st", Ordinal(types.9).to_string());
        assert_eq!("1st", Ordinal(types.10).to_string());
        assert_eq!("1st", Ordinal(types.11).to_string());

        assert_eq!("1st", Ordinal(big_types.0).to_string());
        assert_eq!("1st", Ordinal(big_types.1).to_string());
    }

    #[test]
    fn test_to_primitive_method() {
        let types: (
            i8,
            i16,
            i32,
            i64,
            i128,
            isize,
            u8,
            u16,
            u32,
            u64,
            u128,
            usize,
        ) = (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1);

        assert_eq!(1 as i8, Ordinal(types.0).to_i8());
        assert_eq!(1 as i16, Ordinal(types.1).to_i16());
        assert_eq!(1 as i32, Ordinal(types.2).to_i32());
        assert_eq!(1 as i64, Ordinal(types.3).to_i64());
        assert_eq!(1 as i128, Ordinal(types.4).to_i128());
        assert_eq!(1 as isize, Ordinal(types.5).to_isize());
        assert_eq!(1 as u8, Ordinal(types.6).to_u8());
        assert_eq!(1 as u16, Ordinal(types.7).to_u16());
        assert_eq!(1 as u32, Ordinal(types.8).to_u32());
        assert_eq!(1 as u64, Ordinal(types.9).to_u64());
        assert_eq!(1 as u128, Ordinal(types.10).to_u128());
        assert_eq!(1 as usize, Ordinal(types.11).to_usize());

        assert_eq!(1 as i8, Ordinal(types.0).to_primitive());
        assert_eq!(1 as i16, Ordinal(types.1).to_primitive());
        assert_eq!(1 as i32, Ordinal(types.2).to_primitive());
        assert_eq!(1 as i64, Ordinal(types.3).to_primitive());
        assert_eq!(1 as i128, Ordinal(types.4).to_primitive());
        assert_eq!(1 as isize, Ordinal(types.5).to_primitive());
        assert_eq!(1 as u8, Ordinal(types.6).to_primitive());
        assert_eq!(1 as u16, Ordinal(types.7).to_primitive());
        assert_eq!(1 as u32, Ordinal(types.8).to_primitive());
        assert_eq!(1 as u64, Ordinal(types.9).to_primitive());
        assert_eq!(1 as u128, Ordinal(types.10).to_primitive());
        assert_eq!(1 as usize, Ordinal(types.11).to_primitive());
    }

    #[test]
    fn test_to_ordinal_method() {
        let types: (
            i8,
            i16,
            i32,
            i64,
            i128,
            isize,
            u8,
            u16,
            u32,
            u64,
            u128,
            usize,
        ) = (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1);

        assert_eq!(Ordinal(types.0), types.0.to_ordinal());
        assert_eq!(Ordinal(types.1), types.1.to_ordinal());
        assert_eq!(Ordinal(types.2), types.2.to_ordinal());
        assert_eq!(Ordinal(types.3), types.3.to_ordinal());
        assert_eq!(Ordinal(types.4), types.4.to_ordinal());
        assert_eq!(Ordinal(types.5), types.5.to_ordinal());
        assert_eq!(Ordinal(types.6), types.6.to_ordinal());
        assert_eq!(Ordinal(types.7), types.7.to_ordinal());
        assert_eq!(Ordinal(types.8), types.8.to_ordinal());
        assert_eq!(Ordinal(types.9), types.9.to_ordinal());
        assert_eq!(Ordinal(types.10), types.10.to_ordinal());
        assert_eq!(Ordinal(types.11), types.11.to_ordinal());

        assert_eq!("1st", types.0.to_ordinal().to_string());
        assert_eq!("1st", types.1.to_ordinal().to_string());
        assert_eq!("1st", types.2.to_ordinal().to_string());
        assert_eq!("1st", types.3.to_ordinal().to_string());
        assert_eq!("1st", types.4.to_ordinal().to_string());
        assert_eq!("1st", types.5.to_ordinal().to_string());
        assert_eq!("1st", types.6.to_ordinal().to_string());
        assert_eq!("1st", types.7.to_ordinal().to_string());
        assert_eq!("1st", types.8.to_ordinal().to_string());
        assert_eq!("1st", types.9.to_ordinal().to_string());
        assert_eq!("1st", types.10.to_ordinal().to_string());
        assert_eq!("1st", types.11.to_ordinal().to_string());
    }
}
