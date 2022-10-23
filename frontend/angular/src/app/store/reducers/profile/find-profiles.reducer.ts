import { Action, createReducer, on } from '@ngrx/store';
import {
  RequestFindProfilesAction,
  RequestFindProfilesOkAction,
  RequestFindProfilesErrorAction
} from "../../actions/profile/find-profiles.action";


export interface IFindProfilesState {
  error: boolean,
  loading: boolean,
  profileNames: string[],
}

const initialState: IFindProfilesState = {
  error: false,
  loading: false,
  profileNames: [],
}

const reducer = createReducer(
  initialState,
  on (RequestFindProfilesAction, (state, action) => {
    return {...state, error: false, loading: true  }
  }),
  on (RequestFindProfilesOkAction, (state, action) => {
    return {...state,
              loading: false,
              error: false,
              profileNames: action.payload.findProfilesResponse.profileNames }
  }),
  on (RequestFindProfilesErrorAction, (state, action) => {
    return {...state, loading: false, error: true, filteredProfiles: [] }
  })
);

export function findProfilesReducer(state: IFindProfilesState | undefined, action: Action) {
  return reducer(state, action);
}

