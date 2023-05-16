use rand::Rng;

pub fn match_message(msg: String) -> Option<&'static str> {
    let answer = match msg.as_str() {
        "!help" => "Not fully developed, please wait for future version!",
        "김범준" => "죽어",
        "히히" => "히히",
        "ㅋㅋ" => "ㅋㅋ",
        "심심하다" => "스타레일 해",
        "용꼬야" => "말레니아 언제잡음",
        "뭐먹지" => {
            let ran_num = rand::thread_rng().gen_range(0..11);
            match ran_num {
                0 => "짜파게티", 1 => "고르곤졸라 파스타", 2 => "삼겹살",
                3 => "피자", 4 => "옥수수스프", 5 => "냉면", 6 => "토스트",
                7 => "시리얼", 8 => "김치볶음밥", 9 => "부대찌개", _ => "술",
            }
        },
        "?" => "?",
        _ => return None,
    };
    return Some(answer);
}