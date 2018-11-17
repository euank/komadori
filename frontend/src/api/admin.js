import config from '../config';

class AdminAPI {
  static bootstrap(token) {
    return fetch(`${config.api}/admin/bootstrap`, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      mode: 'cors',
      body: JSON.stringify({
        token,
      }),
    }).then((resp) => {
      if (!resp.ok) {
        throw new Error(`admin bootstrap error: ${resp.status}`);
      }
      return resp.json();
    }).then((resp) => {
      if (resp) {
        return {};
      } else if (resp.status) {
        throw new Error(`Error status: ${resp.message}`);
      } else {
        throw new Error('unrecognized status');
      }
    });
  }

  static listUsers() {
    return fetch(`${config.api}/admin/users`, {
      method: 'GET',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      mode: 'cors',
    }).then((resp) => {
      if (!resp.ok) {
        throw new Error(`admin listusers error: ${resp.status}`);
      }
      return resp.json();
    }).then(resp => resp.users);
  }
  static listGroups() {
    return fetch(`${config.api}/group/listAll`, {
      method: 'GET',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      mode: 'cors',
    }).then((resp) => {
      if (!resp.ok) {
        throw new Error(`admin listgroups error: ${resp.status}`);
      }
      return resp.json();
    });
  }

  static createGroup(group) {
    return fetch(`${config.api}/group/create`, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      mode: 'cors',
      body: JSON.stringify(group),
    }).then((resp) => {
      if (!resp.ok) {
        throw new Error(`admin creategroup error: ${resp.status}`);
      }
      return resp.json();
    });
  }
}

export default AdminAPI;
