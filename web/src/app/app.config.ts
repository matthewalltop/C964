import { ApplicationConfig } from '@angular/core';
import { provideRouter, withComponentInputBinding } from '@angular/router';

import { routes } from './app.routes';
import { HttpEvent, HttpHandlerFn, HttpRequest, provideHttpClient, withInterceptors } from '@angular/common/http';
import { Observable } from 'rxjs';



export function corsInterceptor(req: HttpRequest<unknown>, next: HttpHandlerFn): Observable<HttpEvent<unknown>> {
  req.headers.set('Access-Control-Allow-Origin', '*')
             .set('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
  return next(req);
}


export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes, withComponentInputBinding()),
    provideHttpClient(withInterceptors([corsInterceptor]))
  ]
};
