import React, { Component } from 'react';
import PropTypes from 'prop-types';

class AdminManageGroups extends Component {
  constructor() {
    super();
    this.state = {
      hideCreate: true,
      createGroup: {
        name: '',
        public: true,
        description: '',
      },
    };

    this.toggleHideCreate = this.toggleHideCreate.bind(this);
    this.handleFormChange = this.handleFormChange.bind(this);
    this.createGroup = this.createGroup.bind(this);
  }

  componentDidMount() {
    this.props.adminListGroups();
  }

  toggleHideCreate() {
    this.setState({
      hideCreate: !this.state.hideCreate,
    });
  }

  handleFormChange(ev) {
    const { target } = ev;
    const name = target.getAttribute('name');
    const formName = target.form.getAttribute('name');
    const value = target.type === 'checkbox' ? target.checked : target.value;

    this.setState({
      [formName]: {
        ...this.state[formName],
        [name]: value,
      },
    });
  }

  createGroup(ev) {
    try {
      this.props.adminCreateGroup(this.state.createGroup);
    } catch (ex) {
      console.log(ex);
      //
    }
    ev.preventDefault();
  }

  render() {
    if (!this.props.groups) {
      return <h3>Loading...</h3>;
    }
    const cg = this.state.createGroup;
    console.log(cg);
    return (
      <div>
        <h3>Groups</h3>
        <button onClick={this.toggleHideCreate}>Create New Group</button> <br />
        <div className={this.state.hideCreate ? 'hidden' : ''}>
          <form name="createGroup" onSubmit={this.createGroup}>
            <label htmlFor="groupName">
              Group Name:
              <input id="groupName" type="text" name="name" value={cg.name} onChange={this.handleFormChange} />
            </label> <br />
            <label htmlFor="groupPublic">
              Public:
              <input id="groupPublic" checked={cg.public} type="checkbox" name="public" onChange={this.handleFormChange} />
            </label> <br />
            <label htmlFor="groupDescription">
              Description:
              <input id="groupDescription" type="textarea" name="description" value={cg.description} onChange={this.handleFormChange} />
            </label> <br />
            <input type="submit" value="Submit" />
          </form>  <br /><br />
        </div>

        <span>TODO: links </span>
        <h4>Group list</h4>
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
  adminCreateGroup: PropTypes.func.isRequired,
};
AdminManageGroups.defaultProps = {
  groups: null,
};

export default AdminManageGroups;
