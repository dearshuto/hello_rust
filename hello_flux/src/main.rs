use std::collections::VecDeque;

enum Action {
    UpdateTranslation,
    UpdateRotation,
}

trait IStore {
    fn apply(&mut self, action: &Action);
}

struct Dispatcher<TStore: IStore> {
    actions: VecDeque<Action>,

    store: TStore,
}

impl<TStore: IStore> Dispatcher<TStore> {
    pub fn new(store: TStore) -> Self {
        Self {
            actions: VecDeque::new(),
            store,
        }
    }

    pub fn update(&mut self) {
        while let Some(action) = self.actions.pop_front() {
            self.store.apply(&action);
        }
    }

    pub fn push(&mut self, action: Action) {
        self.actions.push_back(action);
    }
}

struct Store;
impl IStore for Store {
    fn apply(&mut self, _actions: &Action) {}
}

fn main() {
    // Dispatcher と Store
    let store = Store {};
    let mut dispatcher = Dispatcher::new(store);

    // Action を Dispacther に登録
    // 実際はここが並列に積まれる
    dispatcher.push(Action::UpdateTranslation);
    dispatcher.push(Action::UpdateRotation);

    // Action を Store に通知する
    // Store 内でさらに
    //
    // 実際はこれが UI スレッドで実行される
    dispatcher.update();
}
