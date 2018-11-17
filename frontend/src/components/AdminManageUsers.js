import React, { Component } from 'react';
import PropTypes from 'prop-types';
import TruncUuid from './TruncUuid';

class AdminManageUsers extends Component {
  componentDidMount() {
    this.props.adminListUsers();
  }

  render() {
    if (this.props.users == null || Object.keys(this.props.users).length === 0) {
      return <h3>Loading...</h3>;
    }
    console.log(this.props.users);
    return (
      <div>
        <h3>Users</h3>
        <table>
          <thead>
            <tr>
              <th>Username</th>
              <th>Uuid</th>
              <th>Email</th>
              <th>Groups</th>
            </tr>
          </thead>
          <tbody>
            {
              Object.values(this.props.users).map(u => (
                <tr key={u.uuid}>
                  <td>{u.username}</td>
                  <td><TruncUuid route="/admin/user" uuid={u.uuid} /></td>
                  <td>{u.email}</td>
                </tr>
              ))
            }
          </tbody>
        </table>
      </div>
    );
  }
}
AdminManageUsers.propTypes = {
  users: PropTypes.object,
  adminListUsers: PropTypes.func.isRequired,
};
AdminManageUsers.defaultProps = {
  users: null,
};

export default AdminManageUsers;
