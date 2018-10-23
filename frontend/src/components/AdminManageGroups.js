import React, { Component } from 'react';
import PropTypes from 'prop-types';

class AdminManageGroups extends Component {
  componentDidMount() {
    this.props.adminListGroups();
  }

  render() {
    if (!this.props.groups) {
      return <h3>Loading...</h3>;
    }
    return (
      <div>
        <h3>Groups</h3>
        <span>TODO: links </span>
        <ul>
          {
            this.props.groups.map(u => <li key={u.uuid}>{u.name}</li>)
          }
        </ul>
      </div>
    );
  }
}
AdminManageGroups.propTypes = {
  groups: PropTypes.array,
  adminListGroups: PropTypes.func.isRequired,
};
AdminManageGroups.defaultProps = {
  groups: null,
};

export default AdminManageGroups;
