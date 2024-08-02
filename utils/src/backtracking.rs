pub trait Action<T> {
    fn execute(&self, state: T) -> T;
    fn undo(&self, state: T) -> T;
    fn is_valid(&self, state: &T) -> bool;
}

pub fn backtracking<S, A, FN, IN, FS>(start: S, mut successors: FN, mut success: FS) -> Option<S>
    where
        A: Action<S>,
        FN: FnMut(&S) -> IN,
        IN: IntoIterator<Item=A>,
        FS: FnMut(&S) -> bool,
{
    step(start, &mut successors, &mut success).ok()
}

fn step<S, A, FN, IN, FS>(mut state: S, successors: &mut FN, success: &mut FS) -> Result<S, S>
    where
        A: Action<S>,
        FN: FnMut(&S) -> IN,
        IN: IntoIterator<Item=A>,
        FS: FnMut(&S) -> bool,
{
    for action in successors(&state) {
        if !action.is_valid(&state) {
            continue;
        }

        state = action.execute(state);
        if success(&state) {
            return Ok(state);
        }

        match step(state, successors, success) {
            Err(s) => {
                state = action.undo(s);
                continue;
            }
            Ok(x) => { return Ok(x); }
        }
    }

    Err(state)
}

pub fn backtracking_iterative<S, A, FN, IN, FS>(start: S, mut successors: FN, mut success: FS) -> Option<S>
    where
        A: Action<S>,
        FN: FnMut(&S) -> IN,
        IN: IntoIterator<Item=A>,
        FS: FnMut(&S) -> bool,
{
    let mut actions_to_apply_stack = vec![successors(&start).into_iter()];
    let mut applied_action_stack: Vec<A> = vec![];
    let mut state = start;
    while let Some(actions) = actions_to_apply_stack.last_mut() {
        let Some(action) = actions.next() else {
            state = applied_action_stack.pop()?.undo(state);
            actions_to_apply_stack.pop();
            continue;
        };

        if !action.is_valid(&state) {
            continue;
        }

        state = action.execute(state);
        if success(&state) {
            return Some(state);
        }

        applied_action_stack.push(action);
        actions_to_apply_stack.push(successors(&state).into_iter());
    }

    None
}