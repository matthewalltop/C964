import { inject, Injectable } from '@angular/core';
import { environment } from '../../environments/environment';
import { map, Observable, tap } from 'rxjs';
import { PlotlyResponse } from '../../models/responses';
import { HttpClient, HttpParams } from '@angular/common/http';

const baseUrl = environment.api.base;


@Injectable({
  providedIn: 'root'
})
export class PlotService {
  private http = inject(HttpClient);

  constructor() { }

  public getPlot(endpoint: string, params?: HttpParams | undefined): Observable<PlotlyResponse> {
      return this.http.get<PlotlyResponse>(`${baseUrl}/${endpoint}`, { params }).pipe(
      map((res) => JSON.stringify(res)),
        map((res) => JSON.parse(res)),
      map((graph) => {
        return {
          traces: graph.traces.map((x: any) => JSON.parse(JSON.stringify(x))),
          layout: JSON.parse(JSON.stringify(graph.layout))
        };
      }),
    );
  }
}
