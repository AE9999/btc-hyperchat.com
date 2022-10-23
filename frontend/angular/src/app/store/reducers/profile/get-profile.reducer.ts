import { Action, createReducer, on } from '@ngrx/store';
import {
 RequestGetProfileAction,
 RequestGetProfileActionResponseErrorAction,
 RequestGetProfileActionResponseOkAction
} from "../../actions/profile/get-profile.actions";
import {IProfile} from "../../../model/profile/profile";

export interface IGetProfileState {
  error: boolean,
  loading: boolean,
  profile: IProfile | undefined,
}

const initialState: IGetProfileState = {
  error: false,
  loading: false,
  profile: undefined,
}

const reducer = createReducer(
  initialState,
  on (RequestGetProfileAction, (state, action) => {
    return {...state, loading: true, error: true, profile: undefined }
  }),
  on (RequestGetProfileActionResponseErrorAction, (state, action) => {
    return {...state, loading: false, error: true, profile: undefined }
  }),
  on (RequestGetProfileActionResponseOkAction, (state, action) => {
    return {...state, loading: false, error: false, profile: action.payload.requestProfileResponse }
  }),
);

export function getProfileReducer(state: IGetProfileState | undefined, action: Action) {
  return reducer(state, action);
}
