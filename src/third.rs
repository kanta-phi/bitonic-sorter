use super::SortOrder;
use std::cmp::Ordering;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // do_sortを呼ぶ代わりに、sort_byを呼ぶようにする
    // is_power_of_twoが呼ぶので、ここからは削除した
    match *order {
        // 昇順ならa.com(b)、降順ならb.com(a)を行う
        SortOrder::Ascending  => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
    where F: Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length x is not a power of two. (x.len() : {})", x.len()))
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;

        // xをバイトニックにソートする
        // 第2引数がtrueのときはcomparatorで示される順序でソート
        do_sort(&mut x[..mid_point], true, comparator);
        // 第2引数がfalseのときはcomparatorとは逆順でソート
        do_sort(&mut x[mid_point..], false, comparator);

        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    // 比較に先立ちforward(bool値)をOrdering値に変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントが
        // swap_conditionと等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    // 親モジュール(first)のsort関数を使用する
    use super::{sort, sort_by};
    use crate::utils::{new_u32_vec, is_sorted_descending, is_sorted_ascending};
    use crate::SortOrder::*;

    #[test]
    fn sort_u32_ascending() {
        // テストデータとしてu32型のベクタを作成し、xに束縛する
        // sort関数によって内容が更新されるので、可変を表すmutキーワードが必要
        let mut x:Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // &mut xは&mut x[..]と書いてもいい
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x:Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        // xの要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        // 文字列のベクタを作り、ソートする
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];   // x.len()が2のべき乗になっていない
        assert!(sort(&mut x, &Ascending).is_err()); // 戻り値はErr
    }

    // 構造体Studentを定義する
    // 構造体は関連する値を1つにまとめたデータ構造。複数のデータフィールドを持つ
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,     // first_name(名前)フィールド。String型
        last_name:  String,     // last_name(苗字)フィールド。String型
        age: u8,                // age(年齢)フィールド。u8型(8ビット符号なし整数)
    }

    impl Student {

        // 関連関数newを定義する
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {

            // 構造体Studentを初期化して返す。Selfはimpl対象の型(Student)の別名
            Self {
                // to_stringメソッドで&str型の引数からString型の値を作る
                first_name: first_name.to_string(),     // first_nameフィールドに値を設定
                last_name:  last_name.to_string(),      // last_nameフィールドに値を設定
                age,    // ageフィールドにage変数の値を設定
                        // フィールドと変数が同じ名前のときは、このように省略形で書ける
            }
        }
    }

    #[test]
    // 年齢で昇順にソートする
    fn sort_students_by_age_ascending() {

        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(
            // sort_by関数でソートする。第2引数はソート順を決めるクロージャ
            // 引数に2つのStudent構造体をとり、ageフィールドの値をcmpメソッドで
            // 比較することで大小を決定する
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        // 結果を検証する
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(sort_by(&mut x,
            // まずlast_nameを比較する
            &|a, b| a.last_name.cmp(&b.last_name)
                // もしlast_nameが等しくない(LessまたはGreater)ならそれを返す
                // last_nameが等しい(Equal)ならfirst_nameを比較する
                .then_with(|| a.first_name.cmp(&b.first_name))), Ok(())
        );
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_large() {
        {
            // 乱数で65,536要素のデータ列を作る(65,536は2の16乗)
            let mut x = new_u32_vec(65536);
            // 昇順にソートする
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            // ソート結果が正しいことを検証する
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted_descending(&x));
        }
    }
}
