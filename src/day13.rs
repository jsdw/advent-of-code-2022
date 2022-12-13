use std::fmt::Write;
use yap::{ Tokens, IntoTokens };
use super::File;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let pairs = parse_input(&file.contents);
    let in_order = pairs.enumerate().filter_map(|(idx, (a,b))| {
        if a < b {
            Some(idx+1)
        } else {
            None
        }
    }).sum();
    Ok(in_order)
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    let pairs = parse_input(&file.contents);
    let mut all_packets: Vec<_> = pairs.flat_map(|(a,b)| [a,b]).collect();

    // lazy; just parse rather than write out the Item stuff..
    let divider1 = parse_line("[[2]]").unwrap();
    let divider2 = parse_line("[[6]]").unwrap();

    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());

    all_packets.sort();

    let pos1 = all_packets.iter().position(|i| i == &divider1);
    let pos2 = all_packets.iter().position(|i| i == &divider2);

    Ok((pos1.unwrap() + 1) * (pos2.unwrap() + 1))
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
enum Item {
    List(Vec<Item>),
    Number(u8)
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Number(n) => n.fmt(f),
            Item::List(l) => {
                f.write_char('[')?;
                for (idx,item) in l.iter().enumerate() {
                    if idx > 0 {
                        f.write_char(',')?;
                    }
                    item.fmt(f)?;
                }
                f.write_char(']')
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.partial_cmp(b),
            (Item::List(a), Item::List(b)) => a.partial_cmp(b),
            (a @ Item::Number(_), Item::List(b)) => vec![a.clone()].partial_cmp(b),
            (Item::List(a), b @ Item::Number(_)) => a.partial_cmp(&vec![b.clone()])
        }
    }
}

fn parse_line(line: &str) -> Option<Item> {
    fn parse_item(toks: &mut impl Tokens<Item=char>) -> Option<Item> {
        yap::one_of!(toks;
            parse_list(toks).map(Item::List),
            parse_number(toks).map(Item::Number)
        )
    }
    fn parse_list(toks: &mut impl Tokens<Item=char>) -> Option<Vec<Item>> {
        if !toks.token('[') {
            return None
        }
        let items: Vec<Item> = toks
            .sep_by(|t| parse_item(t), |t| t.token(','))
            .collect();
        if !toks.token(']') {
            return None
        }
        Some(items)
    }
    fn parse_number(toks: &mut impl Tokens<Item=char>) -> Option<u8> {
        let n: String = toks.tokens_while(|c| c.is_digit(10)).collect();
        n.parse().ok()
    }

    let mut toks = line.trim().into_tokens();
    parse_item(&mut toks)
}

fn parse_input(input: &str) -> impl Iterator<Item=(Item,Item)> + '_ {
    input.trim().split("\n\n").map(|pair| {
        let mut it = pair.split('\n').filter_map(parse_line);
        // blow up if any parsing fails:
        (it.next().unwrap(), it.next().unwrap())
    })
}

#[cfg(test)]
mod test {
    use super::*;

    // Make it easy to construct items for testing
    trait IntoItem {
        fn into_item(self) -> Item;
    }
    impl IntoItem for u8 {
        fn into_item(self) -> Item {
            Item::Number(self)
        }
    }
    impl <const N: usize, I: IntoItem> IntoItem for [I;N] {
        fn into_item(self) -> Item {
            Item::List(self.into_iter().map(|i| i.into_item()).collect())
        }
    }
    impl <I: IntoItem> IntoItem for Vec<I> {
        fn into_item(self) -> Item {
            Item::List(self.into_iter().map(|i| i.into_item()).collect())
        }
    }
    impl IntoItem for Item {
        fn into_item(self) -> Item {
            self
        }
    }
    macro_rules! items {
        ($( $n:expr ),*) => {{
            #[allow(unused_mut)]
            let mut items: Vec<Item> = Vec::new();
            $( items.push($n.into_item()); )*
            items.into_item()
        }}
    }

    #[test]
    fn sorting_examples() {
        let cmps = [
            items![1,1,3,1,1] < items![1,1,5,1,1],
            items![items![1],items![2,3,4]] < items![items![1],4],
            items![9] > items![items![8,7,6]],
            items![items![4,4],4,4] < items![items![4,4],4,4,4],
            items![7,7,7,7] > items![7,7,7],
            items![] < items![3],
            items![items![items![]]] > items![items![]],
            items![1,items![2,items![3,items![4,items![5,6,7]]]],8,9] > items![1,items![2,items![3,items![4,items![5,6,0]]]],8,9]
        ];

        for (idx, r) in cmps.into_iter().enumerate() {
            assert!(r, "failed at {idx}");
        }
    }

    #[test]
    fn pairs_example() {
        let input = "
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        ";

        assert_eq!(star1(File { contents: input.to_string() }).unwrap(), 13)
    }
}