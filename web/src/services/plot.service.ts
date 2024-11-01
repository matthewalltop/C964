import { inject, Injectable } from '@angular/core';
import { environment } from '../environments/environment';
import { map, Observable } from 'rxjs';
import { PlotlyGraph } from '../models/responses';
import { HttpClient, HttpParams } from '@angular/common/http';

const baseUrl = environment.api.base;


@Injectable({
  providedIn: 'root'
})
export class PlotService {
  private http = inject(HttpClient);

  constructor() { }

  public getPlot(endpoint: string, params?: HttpParams | undefined): Observable<PlotlyGraph> {
      // TODO: Add query params.
      return this.http.get<PlotlyGraph>(`${baseUrl}/${endpoint}`, { params }).pipe(
      map((res) => JSON.stringify(res)),
      map((res) => JSON.parse(res)),
      map((graph) => {
        return {
          data: graph.data.map((x: any) => JSON.parse(x)),
          layout: JSON.parse(graph.layout)
        };
      }),
    );
  }
}
