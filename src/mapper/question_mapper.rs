use crate::{dto::game_progress_dto::QuestionDto, model::game_progress::Question};

pub fn to_dto(question: Question) -> QuestionDto {
    QuestionDto {
        question_text: question.question_text,
        answer_1: question.answer_1,
        answer_2: question.answer_2,
        answer_3: question.answer_3,
        answer_4: question.answer_4,
        good_answer_number: question.good_answer_number,
        topic: question.topic,
        remaining_time: question.remaining_time,
    }
}

pub fn to_entity(question: QuestionDto) -> Question {
    Question {
        question_text: question.question_text,
        answer_1: question.answer_1,
        answer_2: question.answer_2,
        answer_3: question.answer_3,
        answer_4: question.answer_4,
        good_answer_number: question.good_answer_number,
        topic: question.topic,
        remaining_time: question.remaining_time,
    }
}
