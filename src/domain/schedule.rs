use chrono::{DateTime, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::sub::schedule_item::{ScheduledAt, ScheduledEvent, ScheduledHabit, ScheduledTask};

/// [FE logic]
/// 1. item은 사용자가 제거할때까지 제거되지 않는다.
/// 2. 매주 월요일, item_schedule를 수정해야하는 부분을 수정한다.
/// 3. 사용자가 item의 특정 요일을 클릭하여 스케쥴을 할당한다.
/// 4. 할당된 스케쥴에 우클릭하여 스케쥴을 수행할 시각을 추가한다.
/// 5. 할당된 스케쥴을 다시 한번 누르면 스케쥴을 할당 해제한다.
/// 6. 사용자가 item 제거시 item_schedule에서 item_id가 일치하는 item 역시 제거한다.
/// [BE logic]
/// user한명당 하나의 schedule을 반드시 가짐. -> schedule자체의 초기화는 가능 but.. 제거 불가능
/// create schedule: 사용자가 가입하자마자 자동으로 생성됨. 각 item들은 모두 빈 벡터.
/// update schedule:
///     add item: 사용자가 선택한 item_id를 받아 해당 item을 items배열에 추가함.
///     remove item: 사용자가 선택한 item_id를 받아 해당 item을 items배열에서 제거함.
///     update item: 사용자가 선택한 item_id를 받아 해당 item의 item_schedule(요일, 시작시각, 종료시각)을 수정함.
/// delete(reset) schedule: 각 items배열을 모두 비움.
/// fetch schedule: 특정 items배열과 item_schedule배열을 반환함.
/// get item: 특정 item_id를 받아 해당 item을 반환함.
/// get all_schedule: 모든 tasks, events, habits를 반환함.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduleModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,

    pub tasks: Vec<ScheduledTask>,
    pub events: Vec<ScheduledEvent>,
    pub habits: Vec<ScheduledHabit>,
    pub scheduled_times: Vec<ScheduledAt>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}
