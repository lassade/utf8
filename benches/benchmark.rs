use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, SamplingMode, Throughput};
//use rand::prelude::*;

const ENGLISH: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin auctor tellus quis nisl pellentesque, eget malesuada sapien commodo. In hac habitasse platea dictumst. Fusce porttitor dolor ac eleifend egestas. Ut aliquet lacus at dolor bibendum, at blandit erat tincidunt. Nam pellentesque sem euismod, mollis lacus in, luctus elit. Sed quis enim congue, laoreet ligula eu, pharetra ipsum. Nam auctor vestibulum semper. Sed vulputate libero vel libero suscipit, eu aliquam velit laoreet. Quisque hendrerit tortor at dolor feugiat varius.";
const ARABIC: &'static str = "لوريم إيبسوم دولور سيت أميت، كونسيكتيور أديبيسسينج إليت. بروين أوكتور تيللس كويس نيسل بيلينتيسكو، إجيت ماليسوادا سابين كومودو. إن هاك هاباسس بلاتيا ديكتومست. فوسس بورتتيتور دولور أس إلييفند إجيستاس. أوت أليكيت لاكوس أت دولور بيبندوم، أت بلانديت إرات تينسينتونت. نام بيلينتيسكو سيم إويسمود، مولليس لاكوس إن، لوكتوس إليت. سيد كويس إنيم كونغو، لاوريت ليغولا إو، بهاريترا إيبسوم.";
const CHINESE: &'static str = "客户很重要，客户会跟着客户走。 据土地的作者说，他是国家球员，他需要一台智能电脑。 据说他就住在这条街上。 痛苦和 需要。 就像湖边的卡车要喝水一样，倒是给开发商拍马屁。 为，柔软的湖中，悲哀的精英。 但是对于那些做作业的人来说， 就是，箭袋本身。 其实，市场总是好的。 但是 需要免费或免费。";

pub fn criterion_benchmark(c: &mut Criterion) {
    fn bench(c: &mut Criterion, group_name: &str, text: &str) {
        let unicode_chars = text.chars().collect::<Vec<char>>();
        let utf8_chars = utf8::Utf8Chars::from_str(text).collect::<Vec<_>>();

        let mut group = c.benchmark_group(group_name);
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.sampling_mode(SamplingMode::Flat);
        group.sample_size(50);
        group.bench_function("core::str::Chars", |b| {
            b.iter(|| {
                for ch in black_box(text).chars() {
                    black_box(ch);
                }
            })
        });
        group.bench_function("utf8::Utf8Chars", |b| {
            b.iter(|| {
                for ch in utf8::Utf8Chars::from_str(black_box(text)) {
                    black_box(ch);
                }
            })
        });
        group.bench_function("utf8::utf8_char_len", |b| {
            b.iter(|| {
                for ch in &utf8_chars {
                    black_box(ch.len());
                }
            })
        });
        group.bench_function("core::char::utf8_len", |b| {
            b.iter(|| {
                for ch in &unicode_chars {
                    black_box(ch.len_utf8());
                }
            })
        });
        group.bench_function("utf8::Utf8Char::is_whitespace", |b| {
            b.iter(|| {
                for ch in &utf8_chars {
                    black_box(ch.is_whitespace());
                }
            })
        });
        group.bench_function("core::char::is_whitespace", |b| {
            b.iter(|| {
                for ch in &unicode_chars {
                    black_box(ch.is_whitespace());
                }
            })
        });
        group.finish();
    }

    bench(c, "english", ENGLISH);
    bench(c, "arabic", ARABIC);
    bench(c, "chinese", CHINESE);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
