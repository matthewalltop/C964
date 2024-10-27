import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { map, Observable, tap } from 'rxjs';
import { environment } from '../environments/environment.development';
import { PlotlyGraph } from '../models/responses';

const baseUrl = environment.api.base;

@Injectable({
  providedIn: 'root'
})
export class DemographicService {
  private http = inject(HttpClient);

  constructor() {
  }

  getDemographics(): Observable<PlotlyGraph> {
    // TODO: This is the generic graph format for everything.
    // The deserialization nonsense can be pulled out into a utility method
    return this.http.get<PlotlyGraph>(`${baseUrl}/demographic`).pipe(
      map((res) => JSON.stringify(res)),
      map((res) => JSON.parse(res)),
      map((graph) => {
        return {
          data: graph.data.map((x: any) => JSON.parse(x)),
          layout: JSON.parse(graph.layout)
        };
      }),
      tap((res) => console.log(res))
    );
  }
}
