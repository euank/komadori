import React, { Component } from 'react';
import PropTypes from 'prop-types';

class EditUser extends Component {
  componentDidMount() {
    this.props.getUser(this.props.uuid);
  }

  render() {
    console.log(this.props.otherUser);
    if (!this.props.otherUser) {
      return <h3>Loading...</h3>;
    }
    const user = this.props.otherUser;
    return (
      <div>
        <h3>{user.username} &mdash; {user.uuid}</h3>
      </div>
    );
  }
}
EditUser.propTypes = {
  uuid: PropTypes.string.isRequired,
  otherUser: PropTypes.object,
  getUser: PropTypes.func.isRequired,
};
EditUser.defaultProps = {
  otherUser: null,
};

export default EditUser;
