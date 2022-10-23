import { Action, createReducer, on } from '@ngrx/store';
import {
  RequestTopProfilesAction,
  RequestTopProfilesResponseOkAction,
  RequestTopProfilesResponseErrorAction
} from "../../actions/profile/top-profiles.actions";


export interface ITopProfilesState {
  error: boolean,
  loading: boolean,
  profileNames: string[],
}

const initialState: ITopProfilesState = {
  error: false,
  loading: false,
  profileNames: [],
}

const reducer = createReducer(
  initialState,
  on (RequestTopProfilesAction, (state, action) => {
    return {...state, loading: true, error: false, profileNames: [] }
  }),
  on (RequestTopProfilesResponseOkAction, (state, action) => {
    return { ...state,
             loading: false,
             error: false,
             profileNames: action.payload.findProfilesResponse.profileNames
    }
  }),
  on (RequestTopProfilesResponseErrorAction, (state, action) => {
    return {...state, loading: false, error: false, profileNames: [] }
  }),
);

export function topProfilesReducer(state: ITopProfilesState | undefined, action: Action) {
  return reducer(state, action);
}
