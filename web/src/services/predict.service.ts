import { HttpClient, HttpParams } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { environment } from '@environment/environment';
import { PredictRequest } from '@models/requests';
import { MLResponse } from '@models/responses';
import { Observable } from 'rxjs';

const baseUrl = environment.api.base;

@Injectable({
  providedIn: 'root'
})
export class PredictService {
  public http = inject(HttpClient);

  public make_prediction(request: PredictRequest): Observable<MLResponse> {
    let params = new HttpParams()
      .set('condition', request.condition)
      .set('gender', request.gender)
      .set('adhd_type', request.adhd_type)
      .set('algorithm', request.algorithm)
      .set('split_ratio', request.split_ratio);

    return this.http.get<MLResponse>(`${baseUrl}/predict`, { params });
  }
}
