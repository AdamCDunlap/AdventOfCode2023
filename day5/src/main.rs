use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum AocError {
    NoSeedsLine,
    InvalidMapName,
    InvalidMapLine,
    DataBeforeMaps,
    NoMapFrom(String),
}

#[derive(Debug, PartialEq, Eq)]
struct MapEntry {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl MapEntry {
    fn contains(&self, src: u64) -> bool {
        src >= self.src_start && src < self.src_start + self.len
    }
    fn translate(&self, src: u64) -> Option<u64> {
        if self.contains(src) {
            Some(src - self.src_start + self.dst_start)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Range {
    first: u64,
    len: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    src_name: String,
    dst_name: String,
    // invariant: Entries' source ranges do not overlap and are sorted by the source range.
    entries: Vec<MapEntry>,
}

#[derive(Debug, PartialEq, Eq)]
struct TranslatePartialResult {
    translated: Range,
    remaining: Range,
}

impl TranslatePartialResult {
    fn new(full_range: &Range, first_result: u64, max_to_translate: Option<u64>) -> Self {
        let len_translated = min_with_none(full_range.len, max_to_translate);

        TranslatePartialResult {
            translated: Range {
                first: first_result,
                len: len_translated,
            },
            remaining: Range {
                first: full_range.first + len_translated,
                len: full_range.len - len_translated,
            },
        }
    }
}

fn min_with_none(a: u64, b: Option<u64>) -> u64 {
    if b.is_none() {
        a
    } else {
        std::cmp::min(a, b.unwrap())
    }
}

impl Map {
    fn translate(&self, src: u64) -> u64 {
        self.entries
            .iter()
            .find_map(|ent| ent.translate(src))
            .unwrap_or(src) // If no matching entries, identity map
    }

    fn translate_partial_identity(
        to_translate: Range,
        next_entry: Option<&MapEntry>,
    ) -> TranslatePartialResult {
        TranslatePartialResult::new(
            &to_translate,
            to_translate.first,
            next_entry.map(|e| e.src_start - to_translate.first),
        )
    }

    /// Translates the first part of `to_translate`, returning one translated range and the rest of the range that was not translated.
    fn translate_partial(&self, to_translate: Range) -> TranslatePartialResult {
        let location = self
            .entries
            .partition_point(|ent| ent.src_start <= to_translate.first);
        if location == 0 {
            // to_translate.first is before any of the map entries, so identity map it up to the start of the first entry.
            Map::translate_partial_identity(to_translate, self.entries.first())
        } else {
            let prev = &self.entries[location - 1];
            if prev.contains(to_translate.first) {
                let len_to_translate = prev.src_start + prev.len - to_translate.first;

                TranslatePartialResult::new(
                    &to_translate,
                    prev.translate(to_translate.first).unwrap(),
                    Some(len_to_translate),
                )
            } else {
                // Identity map up to the next entry, if there is one
                Map::translate_partial_identity(to_translate, self.entries.get(location))
            }
        }
    }

    fn translate_range(&self, to_translate: &Range) -> Vec<Range> {
        let mut remaining = to_translate.clone();
        let mut translated = vec![];
        while remaining.len > 0 {
            let partial = self.translate_partial(remaining);
            translated.push(partial.translated);
            remaining = partial.remaining.clone();
        }
        translated
    }
}

#[cfg(test)]
fn get_test_map() -> Map {
    Map {
        src_name: "".into(),
        dst_name: "".into(),
        entries: [
            MapEntry {
                src_start: 5,
                dst_start: 105,
                len: 10,
            },
            MapEntry {
                src_start: 15,
                dst_start: 215,
                len: 10,
            },
            MapEntry {
                src_start: 30,
                dst_start: 330,
                len: 20,
            },
        ]
        .into(),
    }
}

#[test]
fn translate_partial_range_completely_before_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 1, len: 2 }),
        TranslatePartialResult {
            translated: Range { first: 1, len: 2 },
            remaining: Range { first: 3, len: 0 },
        }
    );
}

#[test]
fn translate_partial_range_partially_before_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 1, len: 50 }),
        TranslatePartialResult {
            translated: Range { first: 1, len: 4 },
            remaining: Range { first: 5, len: 46 },
        }
    );
}

#[test]
fn translate_partial_range_starting_at_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 5, len: 50 }),
        TranslatePartialResult {
            translated: Range {
                first: 105,
                len: 10
            },
            remaining: Range { first: 15, len: 40 },
        }
    );
}

#[test]
fn translate_partial_range_starting_within_first_test() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 6, len: 50 }),
        TranslatePartialResult {
            translated: Range { first: 106, len: 9 },
            remaining: Range { first: 15, len: 41 },
        }
    );
}

#[test]
fn translate_partial_range_starting_at_break_point() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 15, len: 50 }),
        TranslatePartialResult {
            translated: Range {
                first: 215,
                len: 10
            },
            remaining: Range { first: 25, len: 40 },
        }
    );
}

#[test]
fn translate_partial_range_starting_in_empty_range() {
    assert_eq!(
        get_test_map().translate_partial(Range { first: 27, len: 50 }),
        TranslatePartialResult {
            translated: Range { first: 27, len: 3 },
            remaining: Range { first: 30, len: 47 },
        }
    );
}

#[test]
fn translate_range_test() {
    assert_eq!(
        get_test_map().translate_range(&Range { first: 10, len: 30 }),
        [
            Range { first: 110, len: 5 },
            Range {
                first: 215,
                len: 10
            },
            Range { first: 25, len: 5 },
            Range {
                first: 330,
                len: 10
            }
        ]
        .to_vec()
    )
}

fn parse_numbers(num_list: &str) -> Vec<u64> {
    num_list.split(' ').filter_map(|s| s.parse().ok()).collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<Range>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Almanac, AocError> {
        let mut lines = input.lines();

        let seeds_line = lines.next().ok_or(AocError::NoSeedsLine)?;
        let seeds_nums = parse_numbers(
            &seeds_line
                .strip_prefix("seeds: ")
                .ok_or(AocError::NoSeedsLine)?,
        );
        let seeds = seeds_nums
            .chunks(2)
            .map(|vals| Range {
                first: vals[0],
                len: vals[1],
            })
            .collect();

        let mut almanac = Almanac {
            seeds,
            maps: vec![],
        };

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            // If the line ends with map:, then create a new map
            if let Some(map_name) = line.strip_suffix(" map:") {
                let name_parts: Vec<&str> = map_name.trim().split('-').collect();
                if name_parts.len() != 3 {
                    return Err(AocError::InvalidMapName);
                }
                if name_parts[1] != "to" {
                    return Err(AocError::InvalidMapName);
                }
                almanac.maps.push(Map {
                    src_name: name_parts[0].into(),
                    dst_name: name_parts[2].into(),
                    entries: vec![],
                });
            } else {
                // Otherwise, add to the entries of the last map
                let nums = parse_numbers(line);
                if nums.len() != 3 {
                    dbg!(line);
                    dbg!(nums);
                    return Err(AocError::InvalidMapLine);
                }
                almanac
                    .maps
                    .last_mut()
                    .ok_or(AocError::DataBeforeMaps)?
                    .entries
                    .push(MapEntry {
                        dst_start: nums[0],
                        src_start: nums[1],
                        len: nums[2],
                    });
            }
        }

        // Sort entries in each map
        for map in almanac.maps.iter_mut() {
            map.entries.sort_unstable_by_key(|me| me.src_start);
        }

        Ok(almanac)
    }
}

#[test]
fn test_parse_almanac() {
    assert_eq!(
        r#"seeds: 1 2

        seed-to-soil map:
        3 4 5
        6 7 8
        
        soil-to-fertilizer map:
        9 10 11"#
            .parse(),
        Ok(Almanac {
            seeds: vec![Range { first: 1, len: 2 }],
            maps: [
                Map {
                    src_name: "seed".into(),
                    dst_name: "soil".into(),
                    entries: [
                        MapEntry {
                            dst_start: 3,
                            src_start: 4,
                            len: 5
                        },
                        MapEntry {
                            dst_start: 6,
                            src_start: 7,
                            len: 8
                        },
                    ]
                    .into()
                },
                Map {
                    src_name: "soil".into(),
                    dst_name: "fertilizer".into(),
                    entries: [MapEntry {
                        dst_start: 9,
                        src_start: 10,
                        len: 11
                    }]
                    .into()
                },
            ]
            .into()
        })
    )
}

impl Almanac {
    fn find_map(&self, from_type: &str) -> Result<&Map, AocError> {
        self.maps
            .iter()
            .find(|m| m.src_name == from_type)
            .ok_or_else(|| AocError::NoMapFrom(from_type.into()))
    }

    fn translate(
        &self,
        from_type: &str,
        to_type: &str,
        initial_range: Range,
    ) -> Result<Vec<Range>, AocError> {
        let mut cur_type = from_type;
        let mut cur_ranges = vec![initial_range];
        while cur_type != to_type {
            let map = self.find_map(cur_type)?;
            cur_ranges = cur_ranges
                .iter()
                .flat_map(|range| map.translate_range(range))
                .collect();
            cur_type = &map.dst_name;
        }

        Ok(cur_ranges)
    }
}

const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

#[test]
fn test_almanac_translate() {
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "soil",
            Range { first: 79, len: 1 }
        ),
        Ok(vec![Range { first: 81, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 79, len: 1 }
        ),
        Ok(vec![Range { first: 82, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 14, len: 1 }
        ),
        Ok(vec![Range { first: 43, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 55, len: 1 }
        ),
        Ok(vec![Range { first: 86, len: 1 }])
    );
    assert_eq!(
        TEST_INPUT.parse::<Almanac>().unwrap().translate(
            "seed",
            "location",
            Range { first: 13, len: 1 }
        ),
        Ok(vec![Range { first: 35, len: 1 }])
    );
}

// fn part1(input: &str) -> u64 {
//     let almanac: Almanac = input.parse().unwrap();

//     almanac
//         .seeds
//         .iter()
//         .map(|seed| almanac.translate("seed", "location", *seed).unwrap())
//         .min()
//         .unwrap()
// }

// #[test]
// fn test_part1() {
//     assert_eq!(part1(TEST_INPUT), 35);
// }

fn part2(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();

    almanac
        .seeds
        .iter()
        .flat_map(|seed| almanac.translate("seed", "location", seed.clone()).unwrap())
        .map(|loc_range| loc_range.first)
        .min()
        .unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 46);
}

const REAL_INPUT: &str = r#"seeds: 3429320627 235304036 1147330745 114559245 1684000747 468955901 677937579 96599505 1436970021 26560102 3886049334 159534901 936845926 25265009 3247146679 95841652 3696363517 45808572 2319065313 125950148

seed-to-soil map:
583826644 2288418886 120919689
2666741396 3172314277 160907737
416244021 605500997 167582623
779666561 2280573809 7845077
704746333 908146497 74920228
845411123 2565941729 61831565
1527751557 3025978089 146336188
2827649133 2012274036 268299773
259640867 2409338575 156603154
0 2766337222 259640867
787511638 983066725 57899485
907242688 1526828044 485445992
1674087745 237273108 368227889
2180879562 1040966210 485861834
1392688680 773083620 135062877
3095948906 0 237273108
2042315634 2627773294 138563928

soil-to-fertilizer map:
3793729374 3825015981 63222599
1438266078 0 258943930
1292079166 1479426911 146186912
2816531945 2822520060 385496901
1078023340 1741334425 98335224
3856951973 3668871521 111838140
4180611137 3904426682 114356159
0 1958275780 804927654
3968790113 3380257222 80572704
843553208 2801828988 20691072
1724422279 286156201 594958638
3616894031 4018782841 39547646
1697210008 258943930 27212271
3700747997 3472761445 92981377
4101643138 3565742822 50848378
4152491516 3460829926 11931519
976862296 1378265867 101161044
2319380917 881114839 497151028
3380257222 4058330487 236636809
3656441677 3780709661 44306320
864244280 1845657764 112618016
1176358564 1625613823 115720602
4164423035 3888238580 16188102
3202028846 1839669649 5988115
804927654 2763203434 38625554
4049362817 3616591200 52280321

fertilizer-to-water map:
3734704645 4081344261 116089008
474703780 198917265 194664963
1879970783 393582228 36617128
1197375949 1845422975 8989824
466658346 430199356 8045434
2310800010 2638298964 424967672
919557740 1398638397 135905600
3687134144 3790001806 1934146
3470663058 2339708475 216471086
1759740912 1126580170 64763562
896009775 1048686270 23547965
3348634097 3711378096 32073660
3091810489 2234799361 92321214
889058738 1119629133 6951037
2784472328 3743451756 46550050
2843610278 3063266636 231881310
1532834459 674183164 226906453
3273953517 2183514026 51285335
3075491588 4197433269 16318901
3325238852 3791935952 23395245
3689068290 3665741741 45636355
3384455721 3861402055 22006111
3406461832 3295147946 64201226
1412808810 438244790 32132927
3380707757 2556179561 3747964
3850793653 4001077593 80266668
1450046808 470377717 82787651
669368743 1626190154 219232821
1365871086 1072234235 46937724
3184131703 3969255894 11450375
1824504474 553165368 55466309
1206365773 1854412799 159505313
1055463340 906773661 141912609
3931060321 3359349172 306392569
2756139006 3883408166 28333322
193812194 608631677 65551487
2229584884 4213752170 81215126
3195582078 2559927525 78371439
259363681 1191343732 207294665
2183514026 3815331197 46070858
2735767682 3980706269 20371324
888601564 1119171959 457174
1916587911 901089617 5684044
1922271955 1534543997 91646157
4237452890 3911741488 57514406
2831022378 2327120575 12587900
1444941737 193812194 5105071

water-to-light map:
3241790649 0 474458786
1987249042 1535533387 1254541607
289948525 715361304 527138528
3716249435 1242499832 58349573
1051771035 3080023519 694575489
817087053 1300849405 234683982
1746346524 474458786 240902518
0 2790074994 289948525

light-to-temperature map:
2243197897 1858683458 54522139
694397455 637497541 323467072
3781060233 3489670799 513907063
2297720036 960964613 400594644
3448397921 3305645149 184025650
1096611912 137478840 356155107
2159128717 1774614278 84069180
1769470938 1913205597 389657779
1017864527 2302863376 78747385
3632423571 4003577862 148636662
281342434 1361559257 413055021
3393398302 4239967677 54999619
137478840 493633947 143863594
3305645149 4152214524 87753153
1452767019 2381610761 316703919

temperature-to-humidity map:
36297311 0 6725362
2219701249 4240213747 25288799
3578106372 1498215295 38253390
1356053404 1610598521 173674950
446166190 530183876 120299205
813026177 2579337380 225593633
133147960 367690092 61449909
223651565 500781338 29402538
253054103 174578005 193112087
194597869 6725362 29053696
566465395 85143164 1372917
799049686 3045085309 13976491
1529728354 3595799720 487441441
2420467399 2406340018 172679747
2593147146 1784273471 622066547
2244990048 3198005099 143889028
3437740738 4233433470 6780277
43022673 650483081 18483950
2407809136 1485557032 12658263
3215213693 1060498533 222527045
580905107 86516081 88061924
2017169795 1283025578 202531454
3710504883 2579019765 317615
61506623 429140001 71641337
3716287969 769584936 114202886
1328506806 3066097836 27546598
3703468847 3059061800 7036036
769584936 4265502546 29464750
1292525403 883787822 35981403
3710822498 919769225 5465471
3830490855 4083241161 150192309
3616359762 925234696 87109085
2388879076 3093644434 18930060
3529951620 1012343781 48154752
4220837460 1536468685 74129836
3444521015 3112574494 85430605
567838312 35779058 13066795
1038619810 3341894127 253905593
3980683164 2804931013 240154296
0 48845853 36297311

humidity-to-location map:
2609743610 4133079426 108193613
1608826026 1793129808 148682069
3749483646 1957417643 61460641
3216466252 4241273039 49689894
2717937223 1941811877 15605766
3810944287 1225630249 233181949
2124315534 3020458025 302014415
264620169 272336891 68907407
861861645 2458132363 209077985
4044126236 1458812198 250841060
3134115707 2375781818 82350545
1070939630 4290962933 4004363
2733542989 4089410242 43669184
1074943993 3479498485 533882033
2777212173 2018878284 356903534
2426329949 4013380518 76029724
333527576 0 7716722
3266156146 3346379827 130079823
2526267060 1709653258 83476550
1757508095 3476459650 3038835
0 7716722 264620169
1760546930 861861645 363768604
3396235969 2667210348 353247677
2502359673 3322472440 23907387"#;

fn main() {
    println!("part 2: {}", part2(REAL_INPUT));
}
