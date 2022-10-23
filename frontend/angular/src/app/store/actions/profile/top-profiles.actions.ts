import {createAction, props} from "@ngrx/store";
import {IFindProfileNamesResponse} from "../../../model/profile/find-profile-names-response";


export enum TopProfilesActionTypes {
  REQUEST_TOP_PROFILES = '[ACTION] Request Top Profiles',
  REQUEST_TOP_PROFILES_RESPONSE_OK = '[ACTION] Request Top Profiles Response OK',
  REQUEST_TOP_PROFILES_RESPONSE_ERROR = '[ACTION] Request Top Profiles Response Error',
}

export const RequestTopProfilesAction = createAction(
  TopProfilesActionTypes.REQUEST_TOP_PROFILES,
  props<{payload: { } }>()
);

export const RequestTopProfilesResponseOkAction = createAction(
  TopProfilesActionTypes.REQUEST_TOP_PROFILES_RESPONSE_OK,
  props<{payload: { findProfilesResponse: IFindProfileNamesResponse } }>()
);

export const RequestTopProfilesResponseErrorAction = createAction(
  TopProfilesActionTypes.REQUEST_TOP_PROFILES_RESPONSE_ERROR,
  props<{payload: {  } }>()
);

